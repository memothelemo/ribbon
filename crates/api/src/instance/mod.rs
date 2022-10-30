use crate::instance::prelude::*;
use std::ptr::NonNull;

mod base;
mod classes;
mod internal;

pub mod prelude;
pub use base::*;

#[derive(Debug)]
pub struct Instance {
    refs: *mut usize,

    /// Pointer points to any Instance object.
    ///
    /// ## Safety
    /// The value inside this ptr is InstanceType.
    /// We're going to safely cast it and do something
    /// with it dangerously. :)
    pub(crate) ptr: NonNull<libc::c_void>,
    pub(crate) size: usize,
}

impl Clone for Instance {
    fn clone(&self) -> Self {
        unsafe {
            let refs = self.refs;
            let new_size = *refs + 1;
            if new_size > isize::MAX as usize {
                std::process::abort();
            }
            *refs = new_size;

            Self {
                refs: self.refs,
                ptr: self.ptr,
                size: self.size,
            }
        }
    }
}

impl Instance {
    pub(crate) fn from_trait(instance: impl InstanceType) -> Self {
        let size = unsafe { std::mem::size_of_val_raw(&instance) };

        let value = Box::leak(Box::new(instance));
        let refs = Box::leak(Box::new(0));

        let ptr = NonNull::new(value).unwrap();
        let ptr = ptr.as_ptr() as *mut libc::c_void;

        Self {
            refs,
            ptr: NonNull::new(ptr).unwrap(),
            size,
        }
    }

    pub(crate) unsafe fn clone_unsafe(&self) -> Self {
        Self {
            refs: self.refs,
            ptr: self.ptr,
            size: self.size,
        }
    }
}

impl Instance {
    #[must_use]
    pub fn new<T: CreatableInstance>(parent: Option<Instance>) -> Instance {
        T::create(parent)
    }

    // #[must_use]
    pub fn get(&self) -> &dyn InstanceType {
        unsafe {
            let ptr = self.ptr.as_ptr();

            // treat it as a slice right now
            let array = std::slice::from_raw_parts(ptr as *mut u8, self.size);
            std::mem::transmute::<_, &dyn InstanceType>(array)
            // println!("{}", instance.name());
            // instance
        }
    }

    #[must_use]
    pub fn get_mut<T: InstanceType>(&mut self) -> &mut T {
        unsafe { &mut *(self.ptr.as_ptr().cast::<T>()) }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        let refs = unsafe {
            let refs = *self.refs.as_ref().unwrap();
            let new_refs = refs.saturating_sub(1);
            println!(
                "Dropping instance ({:?}): {} -> {}",
                self.ptr, refs, new_refs
            );
            *self.refs.as_mut().unwrap() = new_refs;
            refs
        };
        if refs != 1 {
            return;
        }
        println!("Performing drop from: {:#?}", self.ptr);

        // drop also the children of our family
        //
        // don't worry about the parent, the children will eventually
        // be dropped and prevent from serious memory problems later on
        unsafe {
            // for child in self
            //     .get_mut::<dyn InstanceType>()
            //     .base_mut()
            //     .children
            //     .drain(..)
            // {
            //     drop(child);
            // }
            drop(Box::from_raw(self.ptr.as_ptr()));
        }
    }
}
