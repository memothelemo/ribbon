#[allow(clippy::all)]
mod class_name;
mod internal;
mod lua;

pub mod classes;
pub mod error;
pub mod prelude;
pub mod traits;
pub use class_name::*;
pub use lua::InstanceConstructor;

use classes::private::*;
use internal::InstanceInner;
use traits::{AnyInstance, CreatableInstance, DefaultClassName};
use unsafe_pointers::Owned;

pub struct Instance(Owned<InstanceInner>);

impl Clone for Instance {
    fn clone(&self) -> Self {
        unsafe {
            internal::bump_ref_count(self);
        }
        Self(self.0)
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { internal::handle_instance_drop(self) }
    }
}

impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Instance")
            .field(&self.0.by_ref().ptr().as_ptr())
            .finish()
    }
}

impl PartialEq for Instance {
    fn eq(&self, other: &Self) -> bool {
        self.0 .0.eq(&other.0 .0)
    }
}

impl Instance {
    pub fn new<T: CreatableInstance + 'static>(parent: Option<Instance>) -> Self {
        unsafe {
            let mut inst = Self(InstanceInner::from_value(T::create(), parent));
            let inner = inst.weak_clone();
            set_inner_ptr(inst.any_mut(), inner);
            inst
        }
    }
}

impl Instance {
    pub fn any(&self) -> &dyn AnyInstance {
        unsafe { internal::any_impl(&self.0) }
    }

    pub fn any_mut(&mut self) -> &mut dyn AnyInstance {
        unsafe { internal::any_mut_impl(&mut self.0) }
    }

    pub fn cast<T: AnyInstance + DefaultClassName + 'static>(&self) -> Option<&T> {
        unsafe { internal::cast_impl(&self.0) }
    }

    pub fn cast_mut<T: AnyInstance + DefaultClassName + 'static>(&mut self) -> Option<&mut T> {
        unsafe { internal::cast_impl_mut(&mut self.0) }
    }

    pub(crate) unsafe fn weak_clone(&self) -> Self {
        Self(self.0)
    }
}
