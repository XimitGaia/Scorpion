mod db::DbInterface;

pub trait JobDbInterface {
    pub fn get_db_metadata(&self);
    pub fn create(&self, db: impl DbInterface)->bool;
    pub fn delete(&self, db: impl DbInterface)->bool;
    pub fn update(&self, db: impl DbInterface)->bool;
}