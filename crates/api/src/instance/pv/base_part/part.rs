use crate::instance::prelude::*;

#[derive(Debug)]
pub struct Part {
    base: BasePart,
}

impl Sealed for Part {}
impl BaseInstanceGetter for Part {
    fn base(&self) -> &BaseInstance {
        &self.base.pv.base
    }

    fn base_mut(&mut self) -> &mut BaseInstance {
        &mut self.base.pv.base
    }
}

impl CreatableInstance for Part {
    fn create(_parent: Option<Instance>) -> Instance {
        Instance::from_trait(Self {
            base: BasePart::new("Part"),
        })
    }
}

impl PVInstanceGetter for Part {
    fn pv(&self) -> &PVInstance {
        &self.base.pv
    }

    fn pv_mut(&mut self) -> &mut PVInstance {
        &mut self.base.pv
    }
}
impl PVInstanceType for Part {}

impl BasePartGetter for Part {
    fn base_part(&self) -> &BasePart {
        &self.base
    }

    fn base_part_mut(&mut self) -> &mut BasePart {
        &mut self.base
    }
}
impl BasePartType for Part {}

impl InstanceType for Part {
    fn class_name(&self) -> &'static str {
        "Part"
    }
}
