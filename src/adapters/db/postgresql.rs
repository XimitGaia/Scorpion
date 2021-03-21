#[path = "../../domain/entity/db.rs"] mod Db;
#[path = "../../domain/repository/db_interface.rs"] mod DbInterface;


impl DbInterface for Db as Sqlite {
    fn get_connection();
    fn set_connection(&self, connection) {

    }    
    fn query();
    fn create_entity();
    fn delete_entity();
    fn update_entity();
}
