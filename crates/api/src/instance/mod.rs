pub mod prelude;

pub mod base;
pub mod pv;

pub(crate) mod internal;

use base::InstanceType;

use std::any::{Any, TypeId};
use std::ptr::NonNull;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

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

#[derive(Debug, Clone)]
pub struct Instance {
    ptr: NonNull<dyn InstanceType + 'static>,
}

impl Drop for Instance {
    fn drop(&mut self) {}
}

impl Instance {
    fn from_trait(instance: impl InstanceType) -> Self {
        let value = Box::into_raw(Box::new(instance));
        Self {
            ptr: NonNull::new(value).unwrap(),
        }
    }
}

impl Instance {
    pub fn new<T: CreatableInstance>(parent: Option<Instance>) -> Instance {
        T::create(parent)
    }

    pub fn get(&self) -> &dyn InstanceType {
        unsafe { self.ptr.as_ref() }
    }

    pub fn cast<T: InstanceType>(&self) -> Option<&T> {
        downcast_ref::<T>(self.get())
    }
}
