use super::internal::{BaseInstanceGetter, Sealed};
use super::Instance;

use crate::instance::prelude::*;

#[derive(Debug)]
pub struct BaseInstance {
    pub(crate) id: Ref,
    pub(crate) name: String,
    pub(crate) children: Vec<Instance>,
    pub(crate) parent: Option<Instance>,
}

impl BaseInstance {
    pub(crate) fn new(name: &'static str) -> Self {
        Self {
            id: Ref::new(),
            children: Vec::new(),
            name: name.to_string(),
            parent: None,
        }
    }
}

pub trait InstanceType: Sealed + BaseInstanceGetter + std::any::Any + std::fmt::Debug {
    fn id(&self) -> Ref {
        self.base().id
    }

    fn class_name(&self) -> &'static str;

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
