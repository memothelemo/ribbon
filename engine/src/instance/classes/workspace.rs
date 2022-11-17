use super::prelude::*;

#[derive(Debug)]
pub struct Workspace {
    pub(crate) base: WorldRoot,
}

impl Workspace {
    pub(crate) fn new() -> Self {
        Self {
            base: WorldRoot::new("Workspace", ClassName::Workspace),
        }
    }
}

impl CreatableInstance for Workspace {
    fn create(_parent: Option<Instance>) -> Instance {
        Instance::new_from_trait(Self::new())
    }
}

impl DefaultClassName for Workspace {
    fn default_class_name() -> ClassName {
        ClassName::Workspace
    }
}

impl AnyInstance for Workspace {
    fn base(&self) -> &BaseInstance {
        self.base.base()
    }

    fn base_mut(&mut self) -> &mut BaseInstance {
        self.base.base_mut()
    }
}

impl Sealed for Workspace {}
default_instance_lua_impl!(Workspace);

ribbon_oop::impl_castable!(Workspace, {
    WorldRoot,
    Model,
    PVInstance,
    BaseInstance,
});
