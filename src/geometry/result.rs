///This is not for errors, this is for interactions
pub type Result<(), CauseOfFailure>;

#[derive(Debug)]
pub enum CauseOfFailure {
    BlocksPresent
}