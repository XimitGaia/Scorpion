pub struct Db {
    db_connection: String,
    hash: String
}

pub trait DbInterface {
    fn start(&self);
    // pub fn query(&self);
    // pub fn create_entity(&self);
    // pub fn delete_entity(&self);
    // pub fn update_entity(&self);
}