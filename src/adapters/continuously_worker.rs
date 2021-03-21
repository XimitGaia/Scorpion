#[path = "../domain/entity/continuously_worker.rs"] mod {ContinuouslyWorker, CuntinuouslyWorkerInterface};
#[path = "../domain/entity/job_db_interface.rs"] mod JobDbInterface;
#[path = "../domain/entity/job_process_interface.rs"] mod JobProcessInterface;


impl CuntinuouslyWorkerInterface for ContinuouslyWorker {
}

impl JobDbInterface for ContinuouslyWorker {
    pub fn get_db_metadata(&self);
    pub fn create(&self, db: impl DbInterface)->bool;
    pub fn delete(&self, db: impl DbInterface)->bool;
    pub fn update(&self, db: impl DbInterface)->bool;
}

impl JobProcessInterface for ContinuouslyWorker {
    pub fn run(&self)->bool;
    pub fn kill(&self)->bool;
    pub fn get_memory(&self)->bool;
    pub fn get_cpu(&self)->bool;
    pub fn get_pid(&self)->bool;
}