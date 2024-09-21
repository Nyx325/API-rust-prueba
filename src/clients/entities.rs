use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::entities::{Identifiable, SoftDeletable};

/// Struct que representa un cliente dentro del
/// sistema
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::clients)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Client {
    pub client_id: Option<i32>, // Cambiado a client_id para coincidir con el nombre del campo en el esquema
    pub active: bool,
    pub username: String,
    pub pwd: String,
    pub birth_date: NaiveDate, // Cambiado a birth_date para coincidir con el nombre del campo en el esquema
}

/// Struct que sirve para determinar los parámetros
/// que el usuario envía en un contexto de búsqueda
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientCriteria {
    pub client_id: Option<i32>, // Cambiado a client_id para coincidir con el nombre del campo en el esquema
    pub active: Option<bool>,
    pub username: Option<String>,
    pub pwd: Option<String>,
    pub birth_date: Option<NaiveDate>, // Cambiado a birth_date para coincidir con el nombre del campo en el esquema
}

impl Identifiable<Option<i32>> for Client {
    fn id(&self) -> &Option<i32> {
        &self.client_id
    }
}

impl SoftDeletable for Client {
    fn is_deleted(&self) -> &bool {
        &self.active
    }

    fn set_deleted(&mut self, deleted: bool) {
        self.active = deleted;
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewClient {
    pub username: String,
    pub pwd: String,
    pub birth_date: NaiveDate, // Cambiado a birth_date para coincidir con el nombre del campo en el esquema
}