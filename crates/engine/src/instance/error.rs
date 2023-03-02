use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstanceError {
    #[error("cannot assign parent to itself")]
    ParentAssignedItself,
}
