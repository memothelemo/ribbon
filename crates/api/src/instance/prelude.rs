pub use super::{base::*, pv::*, CreatableInstance, Instance, InstanceCloneError, InstanceType};
pub use crate::{InstanceArena, InstanceRef};

pub(crate) use async_trait::async_trait;
pub(crate) use once_cell::sync::OnceCell;
pub(crate) use rbx_types::*;

pub(crate) use std::sync::Arc;
pub(crate) use tokio::sync::RwLock;

#[macro_export]
macro_rules! get_obj {
    ($var:ident: $arena:expr, $id:expr) => {
        let $var = $arena.lock().await;
        let $var = $var[$id].lock().await;
    };
    (mut $var:ident: $arena:expr, $id:expr) => {
        let _inst_arena = $arena.read().await;
        let mut $var = _inst_arena[$id].lock().await;
    };
}
pub use get_obj;
