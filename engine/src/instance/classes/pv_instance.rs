use super::prelude::*;

#[derive(Debug)]
pub struct PVInstance {
    pub(crate) base: BaseInstance,
}

impl PVInstance {
    pub(crate) fn new(name: &'static str, class: ClassName) -> Self {
        Self {
            base: BaseInstance::new(name, class),
        }
    }
}

impl DefaultClassName for PVInstance {
    fn default_class_name() -> ClassName {
        ClassName::PVInstance
    }
}

impl AnyInstance for PVInstance {
    fn base(&self) -> &BaseInstance {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseInstance {
        &mut self.base
    }
}

impl Sealed for PVInstance {}
default_instance_lua_impl!(PVInstance);

ribbon_oop::impl_castable!(PVInstance, {
    BaseInstance,
});
