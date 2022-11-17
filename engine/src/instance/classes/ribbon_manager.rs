use super::prelude::*;

/// Utility service for Ribbon
#[derive(Debug)]
pub struct RibbonManager {
    pub(crate) base: BaseInstance,
}

impl RibbonManager {
    pub(crate) fn new(name: &'static str, class: ClassName) -> Self {
        Self {
            base: BaseInstance::new(name, class),
        }
    }
}

impl CreatableInstance for RibbonManager {
    fn create(_parent: Option<Instance>) -> Instance {
        Instance::new_from_trait(Self::new("RibbonManager", ClassName::RibbonManager))
    }
}

impl DefaultClassName for RibbonManager {
    fn default_class_name() -> ClassName {
        ClassName::RibbonManager
    }
}

impl AnyInstance for RibbonManager {
    fn base(&self) -> &BaseInstance {
        self.base.base()
    }

    fn base_mut(&mut self) -> &mut BaseInstance {
        self.base.base_mut()
    }
}

impl Sealed for RibbonManager {}
default_instance_lua_impl!(RibbonManager);

ribbon_oop::impl_castable!(RibbonManager, {
    BaseInstance,
});
