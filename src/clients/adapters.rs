use crate::adapters::DieselConnector;
use crate::entities::{Identifiable, Search, SoftDeletable};
use crate::use_cases::{
    Adder, Checker, Finder, LogicalDeleter, PermanentlyDeleter, Repository, Updater,
};

use super::entities::{Client, ClientCriteria, NewClient};
use crate::schema::clients;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::{delete, insert_into, update};

pub struct ClientRepository;

impl ClientRepository {
    pub fn item_is_valid(item: &Client) -> Result<(), String> {
        Ok(())
    }

    pub fn page_size() -> usize {
        15
    }

    pub fn count_clients(
        conn: &mut SqliteConnection, // Ajusta el tipo de conexión según tu base de datos
        criteria: &ClientCriteria,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        // Comienza la consulta
        let mut query = clients::table.into_boxed(); // `into_boxed` para permitir la construcción dinámica de la consulta

        // Aplica los filtros basados en los criterios proporcionados
        if let Some(active) = criteria.active {
            query = query.filter(clients::active.eq(active));
        }
        if let Some(ref username) = criteria.username {
            query = query.filter(clients::username.eq(username));
        }
        if let Some(birth_date) = criteria.birth_date {
            query = query.filter(clients::birth_date.eq(birth_date));
        }

        // Cuenta el número total de registros que cumplen con los criterios
        let total_count = query.count().get_result::<i64>(conn)?;

        Ok(total_count)
    }

    pub fn calculate_total_pages(total_count: i64, page_size: i64) -> i64 {
        if page_size <= 0 {
            return 0; // Manejo de caso donde el tamaño de página es 0 o negativo
        }
        (total_count + page_size - 1) / page_size // Redondea hacia arriba
    }
}

impl Adder<Client> for ClientRepository {
    fn add(item: &Client) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = DieselConnector::establish_connection()?;
        insert_into(clients::table)
            .values(item)
            .execute(&mut conn)?;
        Ok(())
    }
}

impl PermanentlyDeleter<Client, Option<i32>> for ClientRepository {
    fn permanently_delete(item: &Client) -> Result<(), Box<dyn std::error::Error>> {
        if item.id().is_none() {
            return Err("NoIDError: Item sould have an ID".into());
        }

        let mut conn = DieselConnector::establish_connection()?;
        delete(clients::table.filter(clients::client_id.eq(item.id()))).execute(&mut conn)?;
        Ok(())
    }
}

impl Updater<Client, Option<i32>> for ClientRepository {
    fn update(item: &Client) -> Result<(), Box<dyn std::error::Error>> {
        // Usamos una variable auxiliar porque no podemos pasar un
        // Option o algo así al filter
        let id = item
            .id()
            .ok_or_else(|| "Item should have an ID".to_string())?;

        // Establece la conexión
        let mut conn = DieselConnector::establish_connection()?;

        // Realiza la actualización utilizando el ID no nullable
        update(clients::table.filter(clients::client_id.eq(id)))
            .set((
                clients::active.eq(item.active),
                clients::username.eq(&item.username),
                clients::pwd.eq(&item.pwd),
                clients::birth_date.eq(item.birth_date),
            ))
            .execute(&mut conn)?;

        Ok(())
    }
}

impl LogicalDeleter<Client> for ClientRepository {
    fn logically_delete(item: &Client) -> Result<(), Box<dyn std::error::Error>> {
        let id = item
            .id()
            .ok_or_else(|| "Item should have an ID".to_string())?;

        // Establece la conexión
        let mut conn = DieselConnector::establish_connection()?;

        // Realiza la actualización utilizando el ID no nullable
        diesel::update(clients::table.filter(clients::client_id.eq(id)))
            .set(clients::active.eq(false))
            .execute(&mut conn)?;

        Ok(())
    }
}

impl Finder<Client, Option<i32>, ClientCriteria> for ClientRepository {
    fn search_by(
        criteria: &ClientCriteria,
        page_number: usize,
    ) -> Result<crate::entities::Search<ClientCriteria>, Box<dyn std::error::Error>> {
        // Comienza la consulta
        let mut query = clients::table.into_boxed(); // `into_boxed` para permitir la construcción dinámica de la consulta

        // Aplica los filtros basados en los criterios proporcionados
        if let Some(active) = criteria.active {
            query = query.filter(clients::active.eq(active));
        }
        if let Some(ref username) = criteria.username {
            query = query.filter(clients::username.eq(username));
        }
        if let Some(birth_date) = criteria.birth_date {
            query = query.filter(clients::birth_date.eq(birth_date));
        }

        // Aplica el ORDER BY
        query = query.order(clients::username.asc()); // Ajusta el campo según tu necesidad
        let offset = (page_number - 1) * Self::page_size(); // Aplica offset y limit
        query = query.limit(Self::page_size() as i64).offset(offset as i64);

        // Ejecuta la consulta
        let mut conn = DieselConnector::establish_connection()?;
        let result = query.load::<Client>(&mut conn)?;

        let total_count = Self::count_clients(&mut conn, criteria)?;
        let total_pages = Self::calculate_total_pages(total_count, Self::page_size() as i64);

        Ok(Search::new(
            page_number,
            total_pages as usize,
            criteria.clone(),
            serde_json::to_string(&result)?,
        ))
    }

    fn search_by_id(id: usize) -> Result<Option<Client>, Box<dyn std::error::Error>> {
        let id = id as i32;
        // Establish the connection to the database
        let mut conn = DieselConnector::establish_connection()?;

        // Perform the query to find the client by ID
        let query = clients::table
            .into_boxed()
            .filter(clients::client_id.eq(id));
        let result: Vec<Client> = query.load::<Client>(&mut conn)?;

        if let Some(client) = result.get(0) {
            Ok(Some(client.clone()))
        } else {
            Ok(None)
        }
    }
}

impl Repository<Client, Option<i32>, ClientCriteria> for ClientRepository {}

pub async fn add_client(persona: web::Json<NewClient>) -> impl Responder {
    let persona = Client {
        client_id: None,
        active: true,
        username: persona.username.to_string(),
        pwd: persona.pwd.to_string(),
        birth_date: persona.birth_date,
    };

    if let Err(e) = ClientRepository::item_is_valid(&persona) {
        return HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string() }));
    }

    match ClientRepository::add(&persona) {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({"success": true})),
        Err(e) => {
            println!("Error {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string() }))
        }
    }
}
