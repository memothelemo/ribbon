use super::prelude;

pub trait Sealed {}

pub trait BaseInstanceGetter: Sealed {
    fn base(&self) -> &prelude::BaseInstance;
    fn base_mut(&mut self) -> &mut prelude::BaseInstance;
}

pub trait CreatableInstance: Sealed {
    fn create(parent: Option<prelude::Instance>) -> prelude::Instance;
}

pub trait PVInstanceGetter {
    fn pv(&self) -> &prelude::PVInstance;
    fn pv_mut(&mut self) -> &mut prelude::PVInstance;
}

pub trait BasePartGetter {
    fn base_part(&self) -> &prelude::BasePart;
    fn base_part_mut(&mut self) -> &mut prelude::BasePart;
}
