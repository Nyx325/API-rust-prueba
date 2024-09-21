use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, error::Error};

/// Entidad encargada de generar operaciones básicas
/// de base de datos desde diesel
pub struct DieselConnector;
impl DieselConnector {
    /// Esta función se encarga de brindar una conexión
    /// a la base de datos siempre que se haya definido
    /// un `DATABASE_URL` en un archivo .env en la raiz
    /// del proyecto
    pub fn establish_connection() -> Result<SqliteConnection, Box<dyn Error>> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Ok(SqliteConnection::establish(&database_url)?)
    }
}
