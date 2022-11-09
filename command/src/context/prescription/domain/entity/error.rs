use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrescriptionError {
    #[error("prescription invalid, medication with id `{0}` does not exist")]
    MedicationNotExist(String),
    #[error("state machine failed to emit event for command `{0}`")]
    StateMachineTransitionFail(String),
    #[error("unknown error occured")]
    UnknownError,
}
