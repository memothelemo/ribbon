use crate::instance::prelude::*;
use mlua::MetaMethod;
use std::ptr::NonNull;

mod base;
mod classes;
mod internal;

pub mod prelude;
pub use self::base::*;

pub struct InstanceBuilder;

impl mlua::UserData for InstanceBuilder {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "new",
            |_lua, (class_name, parent): (String, Option<Instance>)| match class_name.as_str() {
                "Part" => Ok(Instance::new::<Part>(parent)),
                _ => Err(mlua::Error::external(format!(
                    "'{}' is not a valid class of Instance",
                    class_name
                ))),
            },
        );
    }
}

#[derive(Debug)]
pub struct Instance {
    refs: *mut usize,
    pub(crate) ptr: NonNull<dyn InstanceType>,
}

impl Instance {
    pub fn clear_parent(instance: &mut Self) {
        let base = instance.get_mut();
        let its_id = base.id();
        if let Some(mut parent) = base.parent() {
            let parent = parent.get_mut();
            let position = parent
                .children()
                .iter()
                .position(|v| v.get().id() == its_id)
                .expect("Oh my no! My child is gone!");

            parent.base_mut().children.remove(position);
        }
    }

    pub fn set_parent(instance: &mut Self, mut parent: Instance) {
        // clear out the old parent
        Self::clear_parent(instance);

        unsafe {
            let instance_deref = instance.get_mut();
            instance_deref.base_mut().parent = Some(parent.clone_unsafe());

            let new_parent = parent.get_mut();
            new_parent.base_mut().children.push(instance.clone());
        };
    }
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
        let mut instance = T::create(parent.clone());
        if let Some(parent) = parent {
            Self::set_parent(&mut instance, parent);
        }
        instance
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

        methods.add_meta_method(
            MetaMethod::NewIndex,
            |lua, this, (key, value): (String, mlua::Value)| {
                // SAFETY: We have no choice but to unsafely
                // reference the inner value because the way mlua
                // implements it anyway...
                unsafe { (*this.ptr.as_ptr())._lua_meta_new_index(lua, &key, value) }
            },
        );
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
