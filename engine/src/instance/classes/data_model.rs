use super::prelude::*;

#[derive(Debug)]
pub struct DataModel {
    pub(crate) base: ServiceProvider,
}

impl DataModel {
    pub(crate) fn new(name: &'static str, class: ClassName) -> Self {
        Self {
            base: ServiceProvider::new(name, class),
        }
    }
}

impl CreatableInstance for DataModel {
    fn create(_parent: Option<Instance>) -> Instance {
        Instance::new_from_trait(Self::new("DataModel", ClassName::DataModel))
    }
}

impl DefaultClassName for DataModel {
    fn default_class_name() -> ClassName {
        ClassName::DataModel
    }
}

impl AnyInstance for DataModel {
    fn base(&self) -> &BaseInstance {
        self.base.base()
    }

    fn base_mut(&mut self) -> &mut BaseInstance {
        self.base.base_mut()
    }
}

ribbon_oop::impl_castable!(DataModel, {
    ServiceProvider,
    BaseInstance,
});
