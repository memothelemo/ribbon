pub mod instance;

use id_arena::{Arena, Id};
use instance::InstanceType;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

/// Multi-threaded safe reference of [Instance](Instance).
pub type InstanceRef = Id<Arc<RwLock<dyn InstanceType + 'static>>>;

/// A collection of all stored instances.
pub type InstanceArena = Arc<RwLock<Arena<Arc<RwLock<dyn InstanceType + 'static>>>>>;
