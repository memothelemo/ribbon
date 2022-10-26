use super::prelude::{BaseInstance, Instance};

pub trait Sealed {}

pub trait BaseInstanceGetter: Sealed {
    fn base(&self) -> &BaseInstance;
    fn base_mut(&mut self) -> &mut BaseInstance;
}

pub trait CreatableInstance: Sealed {
    fn create(parent: Option<Instance>) -> Instance;
}
