#![allow(clippy::missing_safety_doc)]
use std::{marker::PhantomData, ptr::NonNull};

#[derive(Debug)]
#[repr(transparent)]
pub struct Owned<T: ?Sized>(pub NonNull<T>);

unsafe impl<T: ?Sized> Send for Owned<T> {}
unsafe impl<T: ?Sized> Sync for Owned<T> {}

impl<T: ?Sized> Copy for Owned<T> {}
impl<T: ?Sized> Clone for Owned<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Owned<T> {
    pub fn new(ptr: Box<T>) -> Self {
        let ptr = Box::leak(ptr);
        unsafe { Self(NonNull::new_unchecked(ptr)) }
    }

    pub unsafe fn boxed(self) -> Box<T> {
        Box::from_raw(self.0.as_ptr())
    }

    pub fn cast<U: CastTo>(self) -> Owned<U::Target> {
        let ptr = self.0.cast::<U::Target>();
        Owned(ptr)
    }

    pub fn by_ref(&self) -> Ref<T> {
        Ref(self.0, PhantomData)
    }

    pub fn by_mut(&self) -> Mut<T> {
        Mut(self.0, PhantomData)
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Ref<'a, T: ?Sized>(NonNull<T>, PhantomData<&'a T>);

impl<'a, T: ?Sized> Copy for Ref<'a, T> {}
impl<'a, T: ?Sized> Clone for Ref<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: ?Sized> Ref<'a, T> {
    pub fn new(ptr: &'a T) -> Self {
        Ref(NonNull::from(ptr), PhantomData)
    }

    pub fn ptr(self) -> NonNull<T> {
        self.0
    }

    pub fn by_mut(self) -> Mut<'a, T> {
        Mut(self.0, PhantomData)
    }

    pub fn cast<U: CastTo>(self) -> Ref<'a, U::Target> {
        Ref(self.0.cast(), PhantomData)
    }

    pub unsafe fn deref(self) -> &'a T {
        &*self.0.as_ptr()
    }

    pub unsafe fn deref_mut(self) -> &'a mut T {
        &mut *self.0.as_ptr()
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Mut<'a, T: ?Sized>(NonNull<T>, PhantomData<&'a mut T>);

impl<'a, T: ?Sized> Copy for Mut<'a, T> {}
impl<'a, T: ?Sized> Clone for Mut<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: ?Sized> Mut<'a, T> {
    pub fn new(ptr: &'a T) -> Self {
        Mut(NonNull::from(ptr), PhantomData)
    }

    pub fn ptr(self) -> NonNull<T> {
        self.0
    }

    pub fn cast<U: CastTo + ?Sized>(self) -> Mut<'a, U::Target> {
        Mut(self.0.cast(), PhantomData)
    }

    pub unsafe fn deref(self) -> &'a T {
        &*self.0.as_ptr()
    }

    pub fn by_ref(self) -> Ref<'a, T> {
        Ref(self.0, PhantomData)
    }

    pub fn extend<'b>(self) -> Mut<'b, T> {
        Mut(self.0, PhantomData)
    }

    pub unsafe fn deref_mut(self) -> &'a mut T {
        &mut *self.0.as_ptr()
    }
}

impl<'a, T> Mut<'a, T> {
    pub unsafe fn read(self) -> T {
        self.0.as_ptr().read()
    }
}

// Force turbofish on all calls of `.cast::<U>()`.
pub trait CastTo {
    type Target;
}

impl<T> CastTo for T {
    type Target = T;
}
