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

#[async_trait]
pub trait CreatableInstance: InstanceType {
    #[doc(hidden)]
    fn create_impl(parent: Option<InstanceRef>) -> Self;

    async fn create(parent: Option<InstanceRef>, arena: InstanceArena) -> InstanceRef
    where
        Self: Sized,
    {
        let instance = Self::create_impl(parent);

        let mut arena = arena.write().await;
        let assigned = arena.alloc(Arc::new(RwLock::new(instance)));

        let mut instance = arena[assigned].write().await;
        instance._base_mut().arena_id.set(assigned).unwrap();
        drop(instance);

        assigned
    }
}

/// Placeholder for [InstanceType].
pub struct Instance(());

impl Instance {
    pub async fn new<T: CreatableInstance>(parent: Option<InstanceRef>, arena: InstanceArena) -> InstanceRef {
        T::create(parent, arena).await
    }
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
pub trait InstanceType: 'static + Send + Sync {
    fn arena_id(&self) -> InstanceRef {
        *self._base().arena_id.get().unwrap()
    }

    /// A low level way to get the base instance.
    #[doc(hidden)]
    fn _base(&self) -> &BaseInstance;

    /// A low level way to get the base instance
    /// with mutable reference that will affect
    /// from the object itself.
    #[doc(hidden)]
    fn _base_mut(&mut self) -> &mut BaseInstance;

    /// Returns the class name of the Instance
    fn class_name(&self) -> &'static str;

    /// Destroys all of an [Instance]'s children.
    async fn clear_all_children(&mut self, arena: prelude::InstanceArena) {
        let arena = arena.read().await;
        let children = &mut self._base_mut().children;
        for child in children.iter() {
            let mut child = arena[*child].write().await;
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
            let arena_ref = arena.read().await;
            let mut child = arena_ref[*child].write().await;
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

    async fn set_parent(&mut self, parent: Option<prelude::InstanceRef>, arena: InstanceArena) {
        println!("Reading from old parent");
        if let Some(old_parent) = self._base().parent {
            let old_parent = &arena.read().await[old_parent];
            let arena_id = *self._base().arena_id.get().unwrap();

            let mut old_parent = old_parent.write().await;
            let children = &mut old_parent._base_mut().children;
            let index = children.iter().position(|x| *x == arena_id).unwrap();

            children.remove(index);
        }
        self._base_mut().parent = parent;

        println!("Reading from parent value");
        if let Some(parent) = parent {
            println!("Accessing arena");
            let parent = &arena.read().await[parent];
            println!("Accessing its parent");
            parent.write().await._base_mut().children.push(*self._base().arena_id.get().unwrap());
        }
    }

    fn parent(&self) -> Option<InstanceRef> {
        self._base().parent
    }
}

impl std::fmt::Debug for dyn InstanceType + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self._base().id.fmt(f)
    }
}
