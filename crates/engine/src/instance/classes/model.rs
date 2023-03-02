use super::{prelude::*, PVInstance};

#[derive(Debug)]
pub struct Model {
    pub base: PVInstance,
}

impl CreatableInstance for Model {
    fn create() -> Self {
        Self {
            base: PVInstance::create("Model", ClassName::Model),
        }
    }
}

impl_rest!(Model, {
    parent = base,
    base = base.base
});
