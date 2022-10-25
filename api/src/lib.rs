pub mod instance;

use id_arena::{Arena, Id};
use instance::Instance;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Multi-threaded safe reference of [Instance](Instance).
pub type InstanceRef = Id<Arc<Mutex<dyn Instance + 'static>>>;

/// A collection of all stored instances.
pub type InstanceArena = Arc<Mutex<Arena<Arc<Mutex<dyn Instance + 'static>>>>>;
