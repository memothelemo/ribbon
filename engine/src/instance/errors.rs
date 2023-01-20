use thiserror::Error;

#[derive(Debug, Error)]
pub enum RemoveChildError {
    #[error("attempt to remove child even if it's not from its children")]
    NotAChild,
}

#[derive(Debug, Error)]
pub enum GetServiceFrror {
    #[error("invalid service class name")]
    NotService,
}

#[derive(Debug, Error)]
pub enum InstanceError {
    #[error(transparent)]
    RemoveChild(#[from] RemoveChildError),
}
