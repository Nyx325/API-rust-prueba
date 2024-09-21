use serde::Serialize;
use std::error::Error;

use crate::entities::{Identifiable, Search, SoftDeletable};
pub trait Adder<Item>
where
    Item: Clone + PartialEq + Serialize,
{
    fn add(item: &Item) -> Result<(), Box<dyn Error>>;
}
pub trait Updater<Item, IdType>
where
    IdType: Serialize,
    Item: Clone + PartialEq + Serialize + Identifiable<IdType>,
{
    fn update(item: &Item) -> Result<(), Box<dyn Error>>;
}
pub trait LogicalDeleter<Item>
where
    Item: Clone + PartialEq + Serialize + SoftDeletable,
{
    fn logically_delete(item: &Item) -> Result<(), Box<dyn Error>>;
}
pub trait PermanentlyDeleter<Item, IdType>
where
    IdType: Serialize,
    Item: Clone + PartialEq + Serialize + Identifiable<IdType>,
{
    fn permanently_delete(item: &Item) -> Result<(), Box<dyn Error>>;
}
pub trait Finder<Model, IdType, Criteria>
where
    Criteria: Clone,
    IdType: Serialize,
    Model: PartialEq + Clone + Identifiable<IdType>,
{
    fn search_by_id(id: usize) -> Result<Option<Model>, Box<dyn Error>>;
    fn search_by(
        criteria: &Criteria,
        page_number: usize,
    ) -> Result<Search<Criteria>, Box<dyn Error>>;
}
pub trait Repository<Item, IdType, Criteria>:
    Adder<Item>
    + Updater<Item, IdType>
    + LogicalDeleter<Item>
    + PermanentlyDeleter<Item, IdType>
    + Finder<Item, IdType, Criteria>
where
    Criteria: Clone,
    IdType: Serialize,
    Item: Clone + PartialEq + Serialize + SoftDeletable + Identifiable<IdType>,
{
}

pub trait Checker<Item, Repository> {
    fn item_is_valid(item: &Item) -> Result<(), Box<dyn Error>>;
}
