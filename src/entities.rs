use serde::{Deserialize, Serialize};

/// `Identifiable` es un trait que indica que un tipo tiene un identificador único.
///
/// Los tipos que implementen este trait deben proporcionar un método para obtener su identificador.
/// El tipo del identificador es genérico y puede ser definido por el usuario.
///
/// # Métodos
///
/// - `id(&self) -> &T`: Devuelve una referencia al identificador único del item.
///
/// # Requisitos
///
/// El tipo `T` del identificador debe implementar `Serialize` para permitir la serialización del identificador.
/// El tipo que implementa este trait también debe implementar `Serialize` para permitir la serialización del item.
///
/// # Ejemplo
///
/// ```rust
/// use serde::Serialize;
///
/// pub trait Identifiable<T>
/// where
///     T: Serialize,
/// {
///     fn id(&self) -> &T;
/// }
///
/// pub struct MyItem {
///     id: String,
///     // Otros campos...
/// }
///
/// impl Identifiable<String> for MyItem {
///     fn id(&self) -> &String {
///         &self.id
///     }
/// }
/// ```
///
pub trait Identifiable<T>
where
    T: Serialize,
    Self: Serialize,
{
    fn id(&self) -> &T;
}

/// `SoftDeletable` es un trait que define la capacidad de un item para ser marcado como eliminado lógicamente.
///
/// Un item que implementa este trait debe proporcionar métodos para marcarlo como eliminado y para verificar su estado de eliminación.
///
/// # Métodos
///
/// - `is_deleted(&self) -> bool`: Devuelve `true` si el item está marcado como eliminado, `false` en caso contrario.
/// - `set_deleted(&mut self, deleted: bool)`: Establece el estado de eliminación del item.
///
/// # Requisitos
///
/// El tipo `Item` debe implementar `Serialize` para permitir la serialización del estado de eliminación.
///
/// # Ejemplo
///
/// ```rust
/// use serde::Serialize;
///
/// pub trait SoftDeletable {
///     fn is_deleted(&self) -> bool;
///     fn set_deleted(&mut self, deleted: bool);
/// }
///
/// pub struct MyItem {
///     deleted: bool,
///     // Otros campos...
/// }
///
/// impl SoftDeletable for MyItem {
///     fn is_deleted(&self) -> bool {
///         self.deleted
///     }
///
///     fn set_deleted(&mut self, deleted: bool) {
///         self.deleted = deleted;
///     }
/// }
/// ```
///
pub trait SoftDeletable
where
    Self: Serialize,
{
    fn is_deleted(&self) -> &bool;
    fn set_deleted(&mut self, deleted: bool);
}

/// `Search` es una estructura que representa el resultado de una búsqueda dentro del sistema.
///
/// Esta estructura encapsula información sobre los resultados de una búsqueda paginada,
/// utilizando un tipo genérico para los criterios de búsqueda y devolviendo los datos en formato JSON.
///
/// # Campos
///
/// - `total_pages`: El número total de páginas disponibles para la búsqueda.
/// - `page`: El número de la página actual en la búsqueda.
/// - `criteria`: Los criterios de búsqueda utilizados para filtrar los resultados. Este es un
///   parámetro genérico que permite adaptar la búsqueda a diferentes tipos de criterios.
/// - `result`: La lista de datos devuelta en formato JSON como una cadena de texto.
///
/// # Ejemplo
///
/// Suponiendo que tenemos una entidad `Client` y una estructura `ClientCriteria` para representar
/// los criterios de búsqueda, la estructura `Search` podría usarse de la siguiente manera:
///
/// ```rust
/// use chrono::NaiveDate;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Serialize, Deserialize)]
/// pub struct Client {
///     pub client_id: Option<i32>,
///     pub username: String,
///     pub pwd: String,
///     pub birth_date: NaiveDate,
/// }
///
/// #[derive(Debug, Serialize, Deserialize)]
/// pub struct ClientCriteria {
///     pub client_id: Option<i32>,
///     pub username: Option<String>,
///     pub pwd: Option<String>,
///     pub birth_date: Option<NaiveDate>,
/// }
///
/// #[derive(Debug, Serialize, Deserialize)]
/// pub struct Search<Criteria> {
///     pub total_pages: usize,
///     pub page: usize,
///     pub criteria: Criteria,
///     pub result: String,
/// }
///
/// // Ejemplo de uso con `Client` y `ClientCriteria`
/// let criteria = ClientCriteria {
///     client_id: Some(1),
///     username: Some("user1".to_string()),
///     pwd: None,
///     birth_date: None,
/// };
///
/// let search_result = Search {
///     total_pages: 5,
///     page: 1,
///     criteria,
///     result: r#"[
///         {"client_id": 1, "username": "user1", "pwd": "password1", "birth_date": "1990-01-01"},
///         {"client_id": 2, "username": "user2", "pwd": "password2", "birth_date": "1992-02-02"}
///     ]"#.to_string(),
/// };
///
/// println!("{:?}", search_result);
/// ```
///
/// En el ejemplo anterior, `search_result` representa un resultado de búsqueda con 5 páginas en total,
/// estando en la primera página, usando un `ClientCriteria` para filtrar la búsqueda y devolviendo una lista
/// de clientes en formato JSON.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Search<Criteria> {
    pub total_pages: usize,
    pub page: usize,
    pub criteria: Criteria,
    pub result: String,
}

impl<Criteria> Search<Criteria> {
    pub fn new(
        page: usize,
        total_pages: usize,
        criteria: Criteria,
        result: String,
    ) -> Search<Criteria> {
        Self {
            total_pages,
            page,
            criteria,
            result,
        }
    }
}
