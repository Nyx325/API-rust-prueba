use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Struct que representa un cliente dentro del
/// sistema
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::clients)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Client {
    pub client_id: Option<i32>, // Cambiado a client_id para coincidir con el nombre del campo en el esquema
    pub username: String,
    pub pwd: String,
    pub birth_date: NaiveDate, // Cambiado a birth_date para coincidir con el nombre del campo en el esquema
}

/// Struct que sirve para determinar los parámetros
/// que el usuario envía en un contexto de búsqueda
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCriteria {
    pub client_id: Option<i32>, // Cambiado a client_id para coincidir con el nombre del campo en el esquema
    pub username: Option<String>,
    pub pwd: Option<String>,
    pub birth_date: Option<NaiveDate>, // Cambiado a birth_date para coincidir con el nombre del campo en el esquema
}
