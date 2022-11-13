use thiserror::Error;

#[derive(Debug, Error)]
pub enum RemoveChildError {
    #[error("attempt to remove child even if it's not from its children")]
    NotAChild,
}

#[derive(Debug, Error)]
pub enum AnyInstanceError {
    #[error(transparent)]
    RemoveChild(#[from] RemoveChildError),
}