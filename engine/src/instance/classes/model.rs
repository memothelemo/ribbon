use super::prelude::*;

#[derive(Debug)]
pub struct Model {
    pub(crate) base: PVInstance,
}

impl Model {
    pub(crate) fn new(name: &'static str, class: ClassName) -> Self {
        Self {
            base: PVInstance::new(name, class),
        }
    }
}

impl CreatableInstance for Model {
    fn create(_parent: Option<Instance>) -> Instance {
        Instance::new_from_trait(Self {
            base: PVInstance::new("Model", ClassName::Model),
        })
    }
}

impl DefaultClassName for Model {
    fn default_class_name() -> ClassName {
        ClassName::Model
    }
}

impl AnyInstance for Model {
    fn base(&self) -> &BaseInstance {
        self.base.base()
    }

    fn base_mut(&mut self) -> &mut BaseInstance {
        self.base.base_mut()
    }
}

ribbon_oop::impl_castable!(Model, {
    PVInstance,
    BaseInstance,
});
