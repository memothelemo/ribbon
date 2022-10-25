pub mod base;
pub mod pv;

pub mod prelude;

use async_trait::async_trait;
use prelude::*;

/// Possible reasons why an [Instance] failed
/// to clone and makes it own unique object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceCloneError {
    NotArchivable,
    OverrideRequired,
}

/// ## Definition from Roblox
/// Instance is the base class for all classes in
/// the Roblox class hierarchy.
///
/// Every other class that the Roblox engine defines
/// inherits all of the members of Instance.
///
/// It is not possible to directly create Instance objects.
///
/// Source: https://create.roblox.com/docs/reference/engine/classes/Instance
#[async_trait]
pub trait Instance: 'static + Send + Sync {
    /// A low level way to get the base instance.
    #[doc(hidden)]
    fn _base(&self) -> &BaseInstanceImpl;

    /// A low level way to get the base instance
    /// with mutable reference that will affect
    /// from the object itself.
    #[doc(hidden)]
    fn _base_mut(&mut self) -> &mut BaseInstanceImpl;

    /// Returns the class name of the Instance
    fn class_name(&self) -> &'static str;

    /// Destroys all of an [Instance]'s children.
    async fn clear_all_children(&mut self, arena: prelude::InstanceArena) {
        let arena = arena.lock().await;
        let children = &mut self._base_mut().children;
        for child in children.iter() {
            let mut child = arena[*child].lock().await;
            child._base_mut().parent = None;
        }
        children.clear();
    }

    /// Create a copy of an object and all its descendants,
    /// ignoring objects that are not [Instance.Archivable](Instance::archivable).
    /// 
    /// It's pretty much impossible to require [Instance] trait
    /// to require Clone trait to be fully implemented but
    /// we have to implement on our own because Rust restricts
    /// any dynamic objects to be "object-safe" traits (known as [E0038](https://doc.rust-lang.org/error-index.html#E0038)).
    async fn clone(&self, arena: prelude::InstanceArena) -> Result<prelude::InstanceRef, InstanceCloneError>;
    
    /// Sets the [Instance.Parent](Instance::parent) property
    /// to nil, locks the Instance.Parent property, disconnects
    /// all connections, and calls Destroyon all children.
    async fn destroy(&mut self, arena: prelude::InstanceArena) {
        // TODO: connection stuff
        let instance = self._base_mut();
        instance.parent = None;

        let children = &mut self._base_mut().children;
        for child in children.iter() {
            let arena_ref = arena.lock().await;
            let mut child = arena_ref[*child].lock().await;
            println!("RACE CONDITION #1");
            child.destroy(arena.clone()).await;
            println!("RACE CONDITION #1 done");
        }
    }

    /// Gets the string reference of [Instance]'s name
    fn name(&self) -> &str {
        &self._base().name
    }

    /// Sets the [Instance]'s name into a new name
    fn set_name(&mut self, name: String) {
        self._base_mut().name = name;
    }

    async fn set_parent(&mut self, parent: prelude::InstanceRef, arena: InstanceArena) {
        self._base_mut().parent = Some(parent);
        let parent = &mut arena.lock().await[parent];
        parent.lock().await._base_mut().children.push(*self._base().arena_id.get().unwrap());
    }
}

impl std::fmt::Debug for dyn Instance + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Instance").field(&self._base().id).finish()
    }
}
