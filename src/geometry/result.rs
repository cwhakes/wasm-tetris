///This is not for errors, this is for interactions
pub type Result = ::std::result::Result<(), CauseOfFailure>;

#[derive(Debug, PartialEq)]
pub enum CauseOfFailure {
    CantFit,
    NoObject,
}