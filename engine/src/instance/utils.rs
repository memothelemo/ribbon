use super::AnyInstance;
use std::any::Any;

pub unsafe fn unsafe_transmute<T: Any>(obj: &dyn AnyInstance) -> &T {
    let unsafe_ptr = obj as &dyn std::any::Any as *const dyn std::any::Any;
    let unsafe_ptr = unsafe_ptr as *const T;
    unsafe { &*unsafe_ptr }
}

pub unsafe fn unsafe_transmute_mut<T: Any>(obj: &mut dyn AnyInstance) -> &mut T {
    let unsafe_ptr = obj as &mut dyn std::any::Any as *mut dyn std::any::Any;
    let unsafe_ptr = unsafe_ptr as *mut T;
    unsafe { &mut *unsafe_ptr }
}
