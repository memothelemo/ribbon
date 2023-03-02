use std::sync::atomic::{AtomicUsize, Ordering};
use unsafe_pointers::{Owned, Ref};

use super::{
    classes::drop_children,
    traits::{AnyInstance, DefaultClassName},
    Instance,
};

pub unsafe fn bump_ref_count(this: &Instance) {
    this.0
        .by_ref()
        .deref_mut()
        .count
        .fetch_add(1, Ordering::Relaxed);
}

pub unsafe fn handle_instance_drop(this: &mut Instance) {
    let inner = this.0.by_ref().deref_mut();
    let old_count = inner.count.fetch_sub(1, Ordering::Release);
    if old_count == 1 {
        drop_children(this.any_mut());

        let inner = this.0.by_ref().deref_mut();
        inner.count.load(Ordering::Acquire);

        // time to drop that object
        (inner.vtable.drop_inner)(this);
    }
}

// Transition from &'static VTable to its own lifetime
//
// Deref and DerefMut won't let me to do with &'static VTable
// because &'1 is different from &'static, so Rust assumes
// that from same implementation in any_impl is actually
// `&'static dyn AnyInstance`
impl std::ops::Deref for Instance {
    type Target = dyn AnyInstance;

    fn deref(&self) -> &Self::Target {
        let ptr = self.0.by_ref();
        unsafe {
            let inner = ptr.deref();
            let vtable = inner.vtable as *const VTable;
            let vtable = &*vtable;
            (vtable.object_ref)(ptr).deref()
        }
    }
}

impl std::ops::DerefMut for Instance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self.0.by_ref();
        unsafe {
            let inner = ptr.deref();
            let vtable = inner.vtable as *const VTable;
            let vtable = &*vtable;
            (vtable.object_ref)(ptr).deref_mut()
        }
    }
}

pub unsafe fn cast_impl_mut<T: AnyInstance + DefaultClassName + 'static>(
    ptr: &mut Owned<InstanceInner>,
) -> Option<&mut T> {
    let ptr = ptr.by_ref();
    let inner = ptr.deref();
    let mut current = (inner.vtable.object_ref)(ptr).deref_mut();

    let original_class_name = current.get_class_name();
    let default_class_name = T::default_class_name();
    if !current.is_a(default_class_name) {
        return None;
    }

    // getting subclasses and branch it downwards
    let subclasses = default_class_name.base_classes();
    for n in subclasses {
        current = current.parent_mut().unwrap_or_else(|| {
            panic!("Cannot get {default_class_name:#?} from {original_class_name:#?}'s subclasses (current = {n:?})");
        });
    }

    // Then cast the AnyInstance stuff
    let current = current as *mut dyn AnyInstance as *mut T;
    Some(&mut *current)
}

pub unsafe fn cast_impl<T: AnyInstance + DefaultClassName + 'static>(
    ptr: &Owned<InstanceInner>,
) -> Option<&T> {
    let ptr = ptr.by_ref();
    let inner = ptr.deref();
    let mut current = (inner.vtable.object_ref)(ptr).deref();

    let original_class_name = current.get_class_name();
    let default_class_name = T::default_class_name();
    if !current.is_a(default_class_name) {
        return None;
    }

    // getting subclasses and branch it downwards
    let subclasses = default_class_name.base_classes();
    for n in subclasses {
        current = current.parent().unwrap_or_else(|| {
            panic!("Cannot get {default_class_name:#?} from {original_class_name:#?}'s subclasses (current = {n:?})");
        });
    }

    // Then cast the AnyInstance stuff
    let current = current as *const dyn AnyInstance as *const T;
    Some(&*current)
}

// Public functions impl
pub unsafe fn any_impl(ptr: &Owned<InstanceInner>) -> &dyn AnyInstance {
    let ptr = ptr.by_ref();
    let inner = ptr.deref();
    (inner.vtable.object_ref)(ptr).deref()
}

pub unsafe fn any_mut_impl(ptr: &mut Owned<InstanceInner>) -> &mut dyn AnyInstance {
    let ptr = ptr.by_ref();
    let inner = ptr.deref();
    (inner.vtable.object_ref)(ptr).deref_mut()
}

/// Used to interact with **actual** contents of every
/// Instance without using generics to where and what Instance
/// contains it. This behavior is the same as anyhow's Error
/// implementation.
///
/// Because Roblox's instance system is based on OOP
/// abstraction, we need to replicate the same thing where
/// in Rust with unsafe code in it.
#[repr(C)]
struct VTable {
    // A test to make sure we're not in trouble with memory management
    #[cfg(debug_assertions)]
    bomb: u64,
    drop_inner: unsafe fn(inst: &mut Instance),
    object_ref: unsafe fn(Ref<InstanceInner>) -> Ref<dyn AnyInstance>,
}

#[repr(C)]
pub struct InstanceInner<T: ?Sized = ()> {
    /// Used to interact with the **actual** contents
    /// of the value without every using generics
    /// to where and what Instance contains it. Just like
    /// in anyhow's Error object for this example.
    vtable: &'static VTable,
    count: AtomicUsize,
    /// The real value of any instance class, it must be
    /// casted with the real type before interacting
    /// with this field.
    value: T,
}

unsafe fn drop_inner<T: AnyInstance + 'static>(inst: &mut Instance) {
    // cast the inner
    let inner = inst.0.cast::<InstanceInner<T>>().boxed();
    drop(inner);
}

impl InstanceInner {
    #[cold]
    pub unsafe fn from_value<T: AnyInstance + 'static>(
        mut value: T,
        parent: Option<Instance>,
    ) -> Owned<Self> {
        let vtable = &VTable {
            #[cfg(debug_assertions)]
            bomb: 23892383298392,
            drop_inner: drop_inner::<T>,
            object_ref: InstanceInner::object_ref::<T>,
        };

        if let Some(parent) = parent {
            value.base_mut().set_parent(Some(parent)).unwrap();
        }

        Owned::new(Box::new(InstanceInner {
            vtable,
            count: AtomicUsize::new(1),
            value,
        }))
        .cast::<InstanceInner>()
    }
}

impl InstanceInner {
    // unsafe fn cast<T: AnyInstance + 'static>(this: Ref<Self>) -> Ref<InstanceInner<T>> {
    //     this.cast::<InstanceInner<T>>()
    // }

    unsafe fn object_ref<T: AnyInstance + 'static>(
        this: Ref<Self>,
    ) -> Ref<dyn AnyInstance + 'static> {
        let unerased = this.cast::<InstanceInner<T>>();
        Ref::new(&unerased.deref().value)
    }
}
