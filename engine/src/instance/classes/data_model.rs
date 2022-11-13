use super::prelude::*;

#[derive(Debug)]
pub struct DataModel {
    pub(crate) base: ServiceProvider,

    // Workspace as it is,
    workspace: Instance,
}

impl DataModel {
    #[allow(unused)]
    pub(crate) fn new(name: &'static str, class: ClassName) -> Self {
        Self {
            base: ServiceProvider::new(name, class),
            workspace: Instance::new::<Workspace>(None),
        }
    }
}

impl DataModel {
    pub fn workspace_as_ptr(&self) -> Instance {
        self.workspace.clone()
    }

    pub fn workspace(&self) -> &Workspace {
        self.workspace.cast().unwrap()
    }

    pub fn workspace_mut(&mut self) -> &mut Workspace {
        self.workspace.cast_mut().unwrap()
    }
}

impl CreatableInstance for DataModel {
    fn create(_parent: Option<Instance>) -> Instance {
        let workspace = Instance::new::<Workspace>(None);
        Instance::new_from_trait(Self {
            base: ServiceProvider::new("DataModel", ClassName::DataModel),
            workspace,
        })
    }

    fn afterwards(&mut self) {
        let data_model_ptr = self.get_self_ptr();
        self.workspace.get_mut().set_parent(Some(data_model_ptr));
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
