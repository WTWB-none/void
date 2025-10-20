use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum FsError {
    #[error("get data error: `{0}`")]
    GetError(String),
    #[error("create error: `{0}`")]
    CreateError(String),
    #[error("move error: `{0}`")]
    MoveError(String),
    #[error("copy error: `{0}`")]
    CopyError(String),
    #[error("rename error: `{0}`")]
    RenameError(String),
    #[error("delete error: `{0}`")]
    DeleteError(String),
    #[error("Scope not allowed!")]
    ScopeNotAllowed,
}
