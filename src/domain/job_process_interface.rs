mod db::DbInterface;

pub trait JobProcessInterface {
    pub fn run(&self)->bool;
    pub fn kill(&self)->bool;
    pub fn get_memory(&self)->bool;
    pub fn get_cpu(&self)->bool;
    pub fn get_pid(&self)->bool;
}