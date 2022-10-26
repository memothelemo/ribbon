use crate::instance::prelude::*;
use once_cell::sync::OnceCell;
use rbx_types::Ref;

#[derive(Debug)]
pub struct BaseInstance {
    // This is sort of required soon...
    pub(crate) id: Ref,
    pub(crate) name: String,
    pub(crate) children: Vec<Instance>,
    pub(crate) parent: Option<Instance>,
}

impl BaseInstance {
    pub(crate) fn new(name: &'static str) -> BaseInstance {
        Self {
            id: Ref::new(),
            children: Vec::new(),
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

    fn children(&self) -> &[Instance] {
        &self.base().children
    }

    fn parent(&self) -> Option<Instance> {
        self.base().parent.clone()
    }
}
