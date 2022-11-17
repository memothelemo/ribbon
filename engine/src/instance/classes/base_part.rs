use super::prelude::*;

#[derive(Debug)]
pub struct BasePart {
    pub(crate) base: PVInstance,
}

impl BasePart {
    pub(crate) fn new(name: &'static str, class: ClassName) -> Self {
        Self {
            base: PVInstance::new(name, class),
        }
    }
}

impl DefaultClassName for BasePart {
    fn default_class_name() -> ClassName {
        ClassName::BasePart
    }
}

impl AnyInstance for BasePart {
    fn base(&self) -> &super::BaseInstance {
        self.base.base()
    }

    fn base_mut(&mut self) -> &mut super::BaseInstance {
        self.base.base_mut()
    }
}

impl Sealed for BasePart {}
default_instance_lua_impl!(BasePart);

ribbon_oop::impl_castable!(BasePart, {
    PVInstance,
    BaseInstance,
});
