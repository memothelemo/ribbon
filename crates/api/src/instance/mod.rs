pub mod prelude;

pub mod base;
pub mod pv;

pub(crate) mod internal;

use base::InstanceType;

use std::any::{Any, TypeId};
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};

use self::internal::CreatableInstance;

pub fn downcast_ref<T: Any>(instance: &dyn InstanceType) -> Option<&T> {
    let t = TypeId::of::<T>();
    let concrete = instance.type_id();
    if t == concrete {
        // SAFETY: caller guarantees that T is the correct type
        Some(unsafe { &*(instance as *const dyn Any as *const T) })
    } else {
        None
    }
}

pub type AnyInstance = dyn InstanceType + 'static;
pub type UnsafeInstancePtr = NonNull<AnyInstance>;

#[derive(Debug)]
pub struct Instance {
    refs: Arc<Mutex<usize>>,
    pub(crate) ptr: UnsafeInstancePtr,
}

impl Clone for Instance {
    fn clone(&self) -> Self {
        let mut mutex = self.refs.lock().unwrap();
        let new_size = *mutex + 1;
        if new_size > isize::MAX as usize {
            std::process::abort();
        }
        *mutex = new_size;
        drop(mutex);

        Self {
            refs: self.refs.clone(),
            ptr: self.ptr,
        }
    }
}

impl Instance {
    fn from_trait(instance: impl InstanceType) -> Self {
        let value = Box::leak(Box::new(instance));
        Self {
            refs: Arc::new(Mutex::new(1)),
            ptr: NonNull::new(value).unwrap(),
        }
    }

    fn clone_unsafe(&self) -> Self {
        Self {
            refs: self.refs.clone(),
            ptr: self.ptr,
        }
    }
}

impl std::ops::Deref for Instance {
    type Target = dyn InstanceType + 'static;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl std::ops::DerefMut for Instance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl Instance {
    pub fn clear_parent(mut instance: Instance) {
        let ptr = instance.ptr;
        let base = instance.get_mut();
        if let Some(mut old_parent) = base.base_mut().parent.take() {
            let old_parent = old_parent.get_mut();
            let position = old_parent
                .children()
                .iter()
                .position(|v| v.ptr == ptr)
                .expect("my child is gone!");

            old_parent.base_mut().children.remove(position);
        }
    }

    pub fn set_parent(instance: &mut Instance, parent: &mut Instance) {
        // clearing out the old parent
        Self::clear_parent(instance.clone());
        let instance_ref = instance.get_mut();
        instance_ref.base_mut().parent = Some(parent.clone_unsafe());

        let new_parent = parent.get_mut();
        new_parent.base_mut().children.push(instance.clone());
    }
}

impl Instance {
    pub fn new<T: CreatableInstance>(parent: Option<Instance>) -> Instance {
        T::create(parent)
    }

    pub unsafe fn ptr(&self) -> *mut AnyInstance {
        self.ptr.as_ptr()
    }

    pub fn get(&self) -> &dyn InstanceType {
        unsafe { self.ptr.as_ref() }
    }

    pub fn get_mut(&mut self) -> &mut dyn InstanceType {
        unsafe { self.ptr.as_mut() }
    }

    pub fn cast<T: InstanceType>(&self) -> Option<&T> {
        downcast_ref::<T>(self.get())
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        let refs = *self.refs.lock().unwrap();
        let new_refs = refs.saturating_sub(1);

        *self.refs.lock().unwrap() = new_refs;
        println!(
            "Dropping instance ({:?}): {} -> {}",
            self.ptr, refs, new_refs
        );
        if refs != 1 {
            return;
        }
        println!("Performing drop from: {:#?}", self.ptr);

        // drop also the children of our family
        //
        // don't worry about the parent, the children will eventually
        // be dropped and prevent from serious memory problems later on
        unsafe {
            for child in self.get_mut().base_mut().children.drain(..) {
                drop(child);
            }
            drop(Box::from_raw(self.ptr.as_ptr()));
        }
    }
}
