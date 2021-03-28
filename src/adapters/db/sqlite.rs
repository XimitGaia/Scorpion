#[path = "../../domain/db.rs"] mod {Db, DbInterface};

mod domain;
use std::path::Path;


impl DbInterface for Db as Sqlite {
    fn start(&self){
        let path = Path::new(format!("../../databases/{}", self.hash));
        self.db_connection =  sqlite::open(path).unwrap();
    },
    // fn query(&self);
    // fn create_entity(&self);
    // fn delete_entity(&self);
    // fn update_entity(&self);
}


