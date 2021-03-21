#[path = "../../domain/db.rs"] mod {Db, DbInterface};


impl DbInterface for Db as Sqlite {
    fn get_connection();
    fn set_connection(&self, connection) {

    }
    fn query();
    fn create_entity();
    fn delete_entity();
    fn update_entity();
}
