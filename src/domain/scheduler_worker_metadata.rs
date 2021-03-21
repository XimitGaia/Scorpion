pub struct SchedulerWorkerMetadata {
    PID: u8,
    cpu_used: u8,
    memory_used: u16,
    current_state: String,
    current_time: u16, 
    last_used: u16
}