mod class_name;
mod lua;
mod traits;

pub mod classes;
pub mod prelude;

pub use class_name::*;
pub use lua::*;
pub use traits::*;

use std::ptr::NonNull;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub struct Instance {
    refs: Arc<AtomicUsize>,
    pub(crate) ptr: NonNull<dyn InstanceType>,
}

impl Clone for Instance {
    fn clone(&self) -> Self {
        // FIXME: Optimize the atomic orderings?
        if self.refs.load(Ordering::SeqCst) > (isize::MAX as usize) {
            std::process::abort();
        }
        self.refs.fetch_add(1, Ordering::SeqCst);

        Self {
            refs: self.refs.clone(),
            ptr: self.ptr,
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        let refs = self.refs.fetch_sub(1, Ordering::SeqCst);
        if refs != 1 {
            return;
        }

        unsafe {
            for child in self.get_mut().base_mut().children.drain(..) {
                drop(child);
            }
            drop(Box::from_raw(self.ptr.as_ptr()));
        }
    }
}

impl PartialEq for Instance {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl Eq for Instance {}

impl Instance {
    pub fn new_from_class(
        from_rust: bool,
        class: &str,
        parent: Option<Instance>,
    ) -> Option<Instance> {
        Some(match class {
            "HttpService" if from_rust => Instance::new::<classes::HttpService>(parent),
            "Part" => Instance::new::<classes::Part>(parent),
            "Script" => Instance::new::<classes::Script>(parent),
            _ => return None,
        })
    }

    pub fn new<T: CreatableInstance>(parent: Option<Instance>) -> Instance {
        let mut instance = T::create(parent.clone());

        // TODO: add safety doc stuff
        unsafe {
            let ptr = instance.clone_unsafe();
            instance.get_mut().base_mut().ptr.set(ptr).unwrap();
        }

        if let Some(parent) = parent {
            instance.get_mut().set_parent(parent);
        }

        instance
    }

    pub(crate) fn new_from_trait(value: impl InstanceType + 'static) -> Self {
        let refs = Arc::new(AtomicUsize::new(1));
        let ptr = Box::leak(Box::new(value));
        let ptr = NonNull::new(ptr).unwrap();

        Self { refs, ptr }
    }
}

impl Instance {
    pub(crate) unsafe fn clone_unsafe(&self) -> Self {
        Self {
            refs: self.refs.clone(),
            ptr: self.ptr,
        }
    }

    pub fn get(&self) -> &dyn InstanceType {
        unsafe { self.ptr.as_ref() }
    }

    pub fn get_mut(&mut self) -> &mut dyn InstanceType {
        unsafe { self.ptr.as_mut() }
    }
}
