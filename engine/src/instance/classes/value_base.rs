use super::prelude::*;

#[derive(Debug)]
pub struct ValueBase {
    pub(crate) base: BaseInstance,
}

impl ValueBase {
    pub(crate) fn new(name: &'static str, class: ClassName) -> Self {
        Self {
            base: BaseInstance::new(name, class),
        }
    }
}

impl DefaultClassName for ValueBase {
    fn default_class_name() -> ClassName {
        ClassName::ValueBase
    }
}

impl AnyInstance for ValueBase {
    fn base(&self) -> &super::BaseInstance {
        self.base.base()
    }

    fn base_mut(&mut self) -> &mut super::BaseInstance {
        self.base.base_mut()
    }
}

ribbon_oop::impl_castable!(ValueBase, {
    BaseInstance,
});
