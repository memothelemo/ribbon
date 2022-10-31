use crate::instance::prelude::*;
use mlua::MetaMethod;
use std::ptr::NonNull;

mod base;
mod classes;
mod internal;

pub mod prelude;
pub use self::base::*;

#[derive(Debug)]
pub struct Instance {
    refs: *mut usize,
    pub(crate) ptr: NonNull<dyn InstanceType>,
}

impl Instance {
    pub(crate) fn from_trait(instance: impl InstanceType) -> Self {
        let value = Box::leak(Box::new(instance));
        let refs = Box::leak(Box::new(1));
        let ptr = NonNull::new(value).unwrap();

        Self { refs, ptr }
    }

    pub(crate) unsafe fn clone_unsafe(&self) -> Self {
        Self {
            refs: self.refs,
            ptr: self.ptr,
        }
    }
}

impl Instance {
    #[must_use]
    pub fn new<T: CreatableInstance>(parent: Option<Instance>) -> Instance {
        T::create(parent)
    }

    #[must_use]
    pub fn get(&self) -> &dyn InstanceType {
        unsafe { self.ptr.as_ref() }
    }

    #[must_use]
    pub fn get_mut(&mut self) -> &mut dyn InstanceType {
        unsafe { self.ptr.as_mut() }
    }
}

impl mlua::UserData for Instance {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |lua, this, key: String| {
            this.get()._lua_meta_index(lua, &key)
        });
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
            for child in self.get_mut().base_mut().children.drain(..) {
                drop(child);
            }
            drop(Box::from_raw(self.ptr.as_ptr()));
            drop(Box::from_raw(self.refs));
        }
    }
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
            }
        }
    }
}
