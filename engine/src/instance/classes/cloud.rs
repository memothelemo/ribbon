use super::prelude::*;

#[derive(Debug)]
pub struct Cloud {
    pub(crate) base: BaseInstance,

    pub cover: f64,
    pub density: f64,
    pub enabled: bool,
}

impl DefaultClassName for Cloud {
    fn default_class_name() -> ClassName {
        ClassName::Cloud
    }
}

impl AnyInstance for Cloud {
    fn base(&self) -> &BaseInstance {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseInstance {
        &mut self.base
    }
}

impl CreatableInstance for Cloud {
    fn create(_parent: Option<Instance>) -> Instance {
        Instance::new_from_trait(Self {
            base: BaseInstance::new("Clouds", ClassName::Cloud),
            cover: 1.,
            density: 1.,
            enabled: true,
        })
    }
}

ribbon_oop::impl_castable!(Cloud, {
    BaseInstance,
});
