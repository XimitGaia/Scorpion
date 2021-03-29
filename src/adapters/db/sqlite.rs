#[path="../../domain/db.rs"]
pub mod db;
use std::path::Path;
pub use crate::domain::db::Db;
pub use db::DbInterface;


impl DbInterface for Db<String> {
    fn start(&self) {
        let path_string: String = format!("../../databases/{}", self.hash);
        let path: &Path = Path::new(&path_string);
        self.db_connection  =  sqlite::open(path).unwrap();
    }
    // fn query(&self);
    // fn create_entity(&self);
    // fn delete_entity(&self);
    // fn update_entity(&self);
}



