mod scheduler_worker_metadata::SchedulerWorkerMetadata;

pub struct SchedulerWorker {
    hash: String,
    base_path: String,
    job_metadata: SchedulerWorkerMetadata,
    comand: Vec<String>,
    db_connection: DbConection, 
    dependencie: Dependencie
}