mod db_interface::DbInterface;

pub trait JobInterface {
    pub fn get_db_metadata(self);
    pub fn crud(self, db: DbInterface, op: String)->bool;
}