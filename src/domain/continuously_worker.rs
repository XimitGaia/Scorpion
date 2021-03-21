mod continuously_worker_metadata::ContinuouslyWorkerMetadata;

pub struct ContinuouslyWorker {
    hash: String,
    base_path: String,
    job_metadata: ContinuouslyWorkerMetadata,
    comand: Vec<String>,
    db_connection: DbConection, 
}
