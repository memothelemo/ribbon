use super::{is_rbx_class_name, is_rbx_service, BaseInstance};
use crate::instance::Instance;

#[derive(Debug)]
pub struct ServiceProvider {
    pub base: BaseInstance,
}

impl ServiceProvider {
    pub fn find_service(&self, class_name: &str) -> Option<Instance> {
        if !is_rbx_class_name(class_name) || !is_rbx_service(class_name) {
            return None;
        }

        if let Some(service) = self.base.find_first_child_of_class(class_name) {
            return Some(service);
        }

        None
    }

    pub fn get_service(&self, class_name: &str) -> Option<Instance> {
        // SAFETY: ptr is initialized upon creating Instance object
        match self.find_service(class_name) {
            Some(n) => Some(n),
            None => unsafe {
                let ptr = self.base.ptr.get_unchecked();
                Instance::new_from_class(true, class_name, Some(ptr.clone()))
            },
        }
    }
}
