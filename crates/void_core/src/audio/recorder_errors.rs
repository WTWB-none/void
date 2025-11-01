use thiserror::Error;

#[derive(Debug, Error)]
pub enum RecorderError {
    #[error("recorder initialization error: `{0}`")]
    InitError(String),
    #[error("recording start error: `{0}`")]
    StartError(String),
    #[error("recording stop error: `{0}`")]
    StopError(String),
    #[error("recorder not initialized properly")]
    NotInitialized,
}
