use crate::instance::prelude::*;

#[derive(Debug)]
pub struct Part {
    base: BaseInstance,
}

impl Sealed for Part {}
impl BaseInstanceGetter for Part {
    fn base(&self) -> &BaseInstance {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseInstance {
        &mut self.base
    }
}
impl CreatableInstance for Part {
    fn create(_parent: Option<Instance>) -> Instance {
        Instance::from_trait(Self {
            base: BaseInstance::new("Part"),
        })
    }
}
impl InstanceType for Part {}
