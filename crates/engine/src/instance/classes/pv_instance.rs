use super::prelude::*;

#[derive(Debug)]
pub struct PVInstance {
    pub base: BaseInstance,
}

impl NoncreatableInstance for PVInstance {
    fn create(name: &'static str, class_name: ClassName) -> Self {
        Self {
            base: BaseInstance::create(name, class_name),
        }
    }
}

impl_rest!(PVInstance, {
    parent = base,
    base = base
});
