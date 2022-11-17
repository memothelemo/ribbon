use super::prelude::*;

#[derive(Debug)]
pub struct WorldRoot {
    pub(crate) base: Model,
}

impl WorldRoot {
    pub(crate) fn new(name: &'static str, class: ClassName) -> Self {
        Self {
            base: Model::new(name, class),
        }
    }
}

impl DefaultClassName for WorldRoot {
    fn default_class_name() -> ClassName {
        ClassName::Model
    }
}

impl AnyInstance for WorldRoot {
    fn base(&self) -> &BaseInstance {
        self.base.base()
    }

    fn base_mut(&mut self) -> &mut BaseInstance {
        self.base.base_mut()
    }
}

impl Sealed for WorldRoot {}
default_instance_lua_impl!(WorldRoot);

ribbon_oop::impl_castable!(WorldRoot, {
    Model,
    PVInstance,
    BaseInstance,
});
