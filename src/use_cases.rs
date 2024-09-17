use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::entities::{Identifiable, Search, SoftDeletable};

/// `Adder` es un trait que define la capacidad de agregar un nuevo item a un repositorio o almacenamiento.
///
/// Los tipos que implementen este trait deben proporcionar la funcionalidad para agregar datos a su destino.
///
/// # Métodos
///
/// - `add(&mut self, item: &Item) -> Result<(), Box<dyn Error>>`: Agrega un nuevo item al repositorio.
///
/// # Requisitos
///
/// El tipo `Item` debe implementar los siguientes traits:
/// - `Clone`: Para permitir la clonación de los items.
/// - `PartialEq`: Para permitir la comparación de igualdad.
/// - `Serialize`: Para permitir la serialización del item si es necesario.
///
/// # Ejemplo
///
/// ```rust
/// use std::error::Error;
///
/// pub struct MyRepository {
///     // Campos internos...
/// }
///
/// impl Adder<String> for MyRepository {
///     fn add(&mut self, item: &String) -> Result<(), Box<dyn Error>> {
///         // Implementación para agregar un String al repositorio
///         println!("Adding item: {}", item);
///         Ok(())
///     }
/// }
/// ```
///
pub trait Adder<Item>
where
    Item: Clone + PartialEq + Serialize,
{
    fn add(&mut self, item: &Item) -> Result<(), Box<dyn Error>>;
}

/// `Updater` es un trait que define la capacidad de actualizar un item existente en un repositorio o almacenamiento.
///
/// Los tipos que implementen este trait deben proporcionar la funcionalidad para modificar datos existentes.
///
/// # Métodos
///
/// - `update(&mut self, item: &Item) -> Result<(), Box<dyn Error>>`: Actualiza un item existente en el repositorio.
///
/// # Requisitos
///
/// El tipo `Item` debe implementar los siguientes traits:
/// - `Clone`: Para permitir la clonación de los items.
/// - `PartialEq`: Para permitir la comparación de igualdad.
/// - `Serialize`: Para permitir la serialización del item si es necesario.
/// - `Identifiable<IdType>`: Para asegurar que cada item pueda ser identificado de manera única mediante un identificador de tipo `IdType`.
///
/// El tipo `IdType` debe implementar el trait `Serialize`, permitiendo la serialización del identificador.
///
/// # Ejemplo
///
/// ```rust
/// use std::error::Error;
/// use serde::Serialize;
///
/// pub trait Identifiable<IdType>
/// where
///     IdType: Serialize,
/// {
///     fn id(&self) -> &IdType;
/// }
///
/// pub struct MyItem {
///     id: String,
///     name: String,
///     // Otros campos...
/// }
///
/// impl Identifiable<String> for MyItem {
///     fn id(&self) -> &String {
///         &self.id
///     }
/// }
///
/// pub struct MyRepository {
///     // Campos internos...
/// }
///
/// impl Updater<MyItem, String> for MyRepository {
///     fn update(&mut self, item: &MyItem) -> Result<(), Box<dyn Error>> {
///         // Implementación para actualizar un MyItem en el repositorio
///         println!("Updating item with ID: {}", item.id());
///         Ok(())
///     }
/// }
/// ```
///
pub trait Updater<Item, IdType>
where
    IdType: Serialize,
    Item: Clone + PartialEq + Serialize + Identifiable<IdType>,
{
    fn update(&mut self, item: &Item) -> Result<(), Box<dyn Error>>;
}

/// `LogicalDeleter` es un trait que define la capacidad de realizar una eliminación lógica de un item en un repositorio o almacenamiento.
///
/// La eliminación lógica implica marcar el item como eliminado sin eliminarlo físicamente del almacenamiento.
/// Para que un item pueda ser eliminado lógicamente, debe implementar el trait `SoftDeletable` que proporciona métodos para indicar el estado de eliminación.
///
/// # Métodos
///
/// - `logically_delete(&mut self, item: &Item) -> Result<(), Box<dyn Error>>`: Marca un item como eliminado lógicamente.
///
/// # Requisitos
///
/// El tipo `Item` debe implementar los siguientes traits:
/// - `Clone`: Para permitir la clonación de los items.
/// - `PartialEq`: Para permitir la comparación de igualdad.
/// - `Serialize`: Para permitir la serialización del item si es necesario.
/// - `SoftDeletable`: Para proporcionar métodos que indiquen el estado de eliminación.
///
/// # Ejemplo
///
/// ```rust
/// use std::error::Error;
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
///
/// pub struct MyRepository {
///     // Campos internos...
/// }
///
/// impl LogicalDeleter<MyItem> for MyRepository {
///     fn logically_delete(&mut self, item: &MyItem) -> Result<(), Box<dyn Error>> {
///         if item.is_deleted() {
///             return Err("Item already deleted".into());
///         }
///         // Implementación para marcar un item como eliminado lógicamente
///         println!("Logically deleting item: {:?}", item);
///         Ok(())
///     }
/// }
/// ```
///
pub trait LogicalDeleter<Item>
where
    Item: Clone + PartialEq + Serialize + SoftDeletable,
{
    fn logically_delete(&mut self, item: &Item) -> Result<(), Box<dyn Error>>;
}

/// `PermanentlyDeleter` es un trait que define la capacidad de realizar una eliminación total de un item en un repositorio o almacenamiento.
///
/// La eliminación total implica eliminar el item físicamente del almacenamiento, sin dejar rastro.
///
/// # Métodos
///
/// - `permanently_delete(&mut self, item: &Item) -> Result<(), Box<dyn Error>>`: Elimina físicamente un item del repositorio.
///
/// # Requisitos
///
/// El tipo `Item` debe implementar los siguientes traits:
/// - `Clone`: Para permitir la clonación de los items.
/// - `PartialEq`: Para permitir la comparación de igualdad.
/// - `Serialize`: Para permitir la serialización del item si es necesario.
///
/// # Ejemplo
///
/// ```rust
/// use std::error::Error;
///
/// pub struct MyRepository {
///     // Campos internos...
/// }
///
/// impl PermanentlyDeleter<String> for MyRepository {
///     fn permanently_delete(&mut self, item: &String) -> Result<(), Box<dyn Error>> {
///         // Implementación para eliminar físicamente un String del repositorio
///         println!("Permanently deleting item: {}", item);
///         Ok(())
///     }
/// }
/// ```
///
pub trait PermanentlyDeleter<Item>
where
    Item: Clone + PartialEq + Serialize,
{
    fn permanently_delete(&mut self, item: &Item) -> Result<(), Box<dyn Error>>;
}

/// `Finder` es un trait que define la capacidad de buscar y encontrar items en un repositorio o almacenamiento.
///
/// Los tipos que implementen este trait deben proporcionar la funcionalidad para buscar items por su identificador
/// y por criterios específicos, además de manejar la paginación.
///
/// # Métodos
///
/// - `page_size(&self) -> usize`: Devuelve el tamaño de la página para los resultados de búsqueda.
/// - `search_by_id(&self, id: usize) -> Result<Option<Model>, Box<dyn Error>>`: Busca un item por su identificador único. Devuelve `Some(item)` si el item es encontrado o `None` si no lo es.
/// - `search_by(&mut self, criteria: &Criteria, page_number: usize) -> Result<Search<Criteria>, Box<dyn Error>>`: Busca items basados en los criterios proporcionados y la página solicitada. Devuelve una instancia de `Search` que contiene los resultados.
///
/// # Requisitos
///
/// El tipo `Model` debe implementar los siguientes traits:
/// - `PartialEq`: Para permitir la comparación de igualdad.
/// - `Clone`: Para permitir la clonación de los items.
/// - `Identifiable<IdType>`: Para asegurar que cada item pueda ser identificado de manera única mediante un identificador de tipo `IdType`.
///
/// El tipo `IdType` debe implementar el trait `Serialize`, permitiendo la serialización del identificador.
///
/// # Ejemplo
///
/// ```rust
/// use std::error::Error;
/// use serde::Serialize;
///
/// pub trait Identifiable<IdType>
/// where
///     IdType: Serialize,
/// {
///     fn id(&self) -> &IdType;
/// }
///
/// pub struct MyItem {
///     id: String,
///     name: String,
///     // Otros campos...
/// }
///
/// impl Identifiable<String> for MyItem {
///     fn id(&self) -> &String {
///         &self.id
///     }
/// }
///
/// pub struct MyRepository {
///     // Campos internos...
/// }
///
/// impl Finder<MyItem, String, MyCriteria> for MyRepository {
///     fn page_size(&self) -> usize {
///         20 // Tamaño de página predeterminado
///     }
///
///     fn search_by_id(&self, id: usize) -> Result<Option<MyItem>, Box<dyn Error>> {
///         // Implementación para buscar un item por ID
///         Ok(None) // Ejemplo de implementación
///     }
///
///     fn search_by(&mut self, criteria: &MyCriteria, page_number: usize) -> Result<Search<MyCriteria>, Box<dyn Error>> {
///         // Implementación para buscar items según criterios
///         Ok(Search::default()) // Ejemplo de implementación
///     }
/// }
/// ```
///
pub trait Finder<Model, IdType, Criteria>
where
    IdType: Serialize,
    Model: PartialEq + Clone + Identifiable<IdType>,
{
    fn page_size(&self) -> usize;
    fn search_by_id(&self, id: usize) -> Result<Option<Model>, Box<dyn Error>>;
    fn search_by(
        &mut self,
        criteria: &Criteria,
        page_number: usize,
    ) -> Result<Search<Criteria>, Box<dyn Error>>;
}

/// `Repository` es un trait que combina capacidades de agregar, actualizar, eliminar lógicamente, eliminar totalmente y buscar items.
///
/// Los tipos que implementen este trait deben proporcionar implementaciones para todos los métodos requeridos por `Adder`, `Updater`, `LogicalDeleter`, `PermanentlyDeleter`, y `Finder`.
///
/// # Requisitos
///
/// El tipo `Item` debe implementar los siguientes traits:
/// - `Clone`: Para permitir la clonación de los items.
/// - `PartialEq`: Para permitir la comparación de igualdad.
/// - `Serialize`: Para permitir la serialización del item si es necesario.
/// - `SoftDeletable`: Solo necesario si el repositorio usa eliminación lógica.
/// - `Identifiable<IdType>`: Para asegurar que cada item pueda ser identificado de manera única.
///
/// El tipo `IdType` debe implementar el trait `Serialize`, permitiendo la serialización del identificador.
///
/// # Ejemplo
///
/// ```rust
/// use std::error::Error;
///
/// pub struct MyRepository {
///     // Campos internos...
/// }
///
/// impl Repository<MyItem, String> for MyRepository {
///     // Implementaciones para todos los métodos requeridos por Adder, Updater, LogicalDeleter, PermanentlyDeleter y Finder.
/// }
/// ```
///
pub trait Repository<Item, IdType, Criteria>:
    Adder<Item>
    + Updater<Item, IdType>
    + LogicalDeleter<Item>
    + PermanentlyDeleter<Item>
    + Finder<Item, IdType, Criteria>
where
    IdType: Serialize,
    Item: Clone + PartialEq + Serialize + SoftDeletable + Identifiable<IdType>,
{
}
