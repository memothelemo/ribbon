use super::{AnyInstance, CreatableInstance, InstanceCastable};

use std::ptr::NonNull;
use std::sync::atomic::Ordering;
use std::sync::{atomic::AtomicUsize, Arc};

pub struct Instance {
    refs: Arc<AtomicUsize>,
    ptr: NonNull<dyn AnyInstance>,
}

impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Instance").field(&self.ptr.as_ptr()).finish()
    }
}

impl Clone for Instance {
    fn clone(&self) -> Self {
        unsafe {
            self.bump_ref();
            self.clone_no_ref()
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        let refs = unsafe { self.debump_ref() };
        if refs != 1 {
            return;
        }

        unsafe {
            self.get_mut().base_mut().drop_children();
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
    pub(crate) fn new_from_trait(value: impl AnyInstance + 'static) -> Self {
        let refs = Arc::new(AtomicUsize::new(1));
        let ptr = NonNull::new(Box::leak(Box::new(value))).unwrap();

        Self { ptr, refs }
    }

    #[allow(unused)]
    pub(crate) unsafe fn bump_ref(&self) {
        if self.refs.load(Ordering::SeqCst) > (isize::MAX as usize) {
            std::process::abort();
        }
        self.refs.fetch_add(1, Ordering::SeqCst);
    }

    #[allow(unused)]
    pub(crate) unsafe fn debump_ref(&self) -> usize {
        self.refs.fetch_sub(1, Ordering::SeqCst)
    }

    pub(crate) unsafe fn clone_no_ref(&self) -> Self {
        Self {
            refs: self.refs.clone(),
            ptr: self.ptr,
        }
    }
}

impl Instance {
    pub fn new<T: CreatableInstance + InstanceCastable>(parent: Option<Instance>) -> Self {
        let mut instance = T::create(parent.clone());

        unsafe {
            let ptr = instance.clone_no_ref();
            instance.get_mut().base_mut().ptr.set(ptr).unwrap();
        }
        instance.get_mut().set_parent(parent);

        unsafe {
            T::after_created(instance.cast_mut::<T>().unwrap());
        }
        instance
    }

    pub fn builder<T: CreatableInstance + InstanceCastable>(
        mut builder: impl FnMut(&mut T),
    ) -> Self {
        let mut instance = Instance::new::<T>(None);
        builder(instance.cast_mut::<T>().unwrap());

        instance
    }
}

impl Instance {
    pub fn cast<T: InstanceCastable>(&self) -> Option<&T> {
        T::downcast(self.get())
    }

    pub fn cast_mut<T: InstanceCastable>(&mut self) -> Option<&mut T> {
        T::downcast_mut(self.get_mut())
    }

    pub fn get(&self) -> &dyn AnyInstance {
        unsafe { self.ptr.as_ref() }
    }

    pub fn get_mut(&mut self) -> &mut dyn AnyInstance {
        unsafe { self.ptr.as_mut() }
    }
}
