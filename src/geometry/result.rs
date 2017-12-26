pub type Result<(), CauseOfFailure>;

#[derive(Debug)]
pub enum CauseOfFailure {
    BlocksPresent
}