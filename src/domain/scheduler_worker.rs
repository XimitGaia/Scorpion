mod scheduler_worker_metadata::SchedulerWorkerMetadata;

pub struct SchedulerWorker {
    hash: String,
    base_path: String,
    job_metadata: SchedulerWorkerMetadata,
    comand: Vec<String>,
    dependencie: Dependencie
}

pub trait CuntinuouslyWorkerInterface{
}