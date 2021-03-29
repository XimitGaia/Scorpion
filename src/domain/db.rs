pub struct Db<String> {
    pub hash: String,
    pub db_connection: sqlite::Connection
}

pub trait DbInterface {
    fn start(&self);
    // pub fn query(&self);
    // pub fn create_entity(&self);
    // pub fn delete_entity(&self);
    // pub fn update_entity(&self);
}