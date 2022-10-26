use crate::instance::prelude::*;
use rbx_types::Ref;

#[derive(Debug)]
pub struct BaseInstance {
    pub(crate) id: Ref,
    pub(crate) name: String,
    pub(crate) parent: Option<Instance>,
}

impl BaseInstance {
    pub(crate) fn new(name: &'static str) -> BaseInstance {
        Self {
            id: Ref::new(),
            name: name.to_string(),
            parent: None,
        }
    }
}

pub trait InstanceType: Sealed + BaseInstanceGetter + std::any::Any {
    fn id(&self) -> Ref {
        self.base().id
    }

    fn name(&self) -> &str {
        &self.base().name
    }

    fn name_mut(&mut self) -> &mut String {
        &mut self.base_mut().name
    }

    fn parent(&self) -> Option<Instance> {
        self.base().parent.clone()
    }
}
