pub trait DbInterface {
    pub fn set_connection();
    fn get_connection();
    pub fn query();
    pub fn create_entity();
    pub fn delete_entity();
    pub fn update_entity();
}
