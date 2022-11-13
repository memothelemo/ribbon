use super::prelude::*;

#[derive(Debug)]
pub struct ServiceProvider {
    pub(crate) base: BaseInstance,
}

impl ServiceProvider {
    pub(crate) fn new(name: &'static str, class: ClassName) -> Self {
        Self {
            base: BaseInstance::new(name, class),
        }
    }
}

impl DefaultClassName for ServiceProvider {
    fn default_class_name() -> ClassName {
        ClassName::ServiceProvider
    }
}

impl AnyInstance for ServiceProvider {
    fn base(&self) -> &BaseInstance {
        self.base.base()
    }

    fn base_mut(&mut self) -> &mut BaseInstance {
        self.base.base_mut()
    }
}

ribbon_oop::impl_castable!(ServiceProvider, {
    BaseInstance,
});
