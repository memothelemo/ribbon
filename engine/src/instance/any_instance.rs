use crate::types::Ref;

use super::classes::BaseInstance;
use super::errors::*;
use super::{ClassName, DefaultClassName, Instance};

pub trait AnyInstance: std::any::Any {
    fn base(&self) -> &BaseInstance;
    fn base_mut(&mut self) -> &mut BaseInstance;

    fn add_child(&mut self, child: Instance) {
        self.base_mut().add_child(child)
    }

    fn children(&self) -> std::slice::Iter<'_, Instance> {
        self.base().children()
    }

    fn class(&self) -> ClassName {
        self.base().class()
    }

    fn clear_parent(&mut self) {
        self.base_mut().clear_parent()
    }

    fn descendants(&self) -> Vec<Instance> {
        self.base().descendants()
    }

    fn get_self_ptr(&self) -> Instance {
        self.base().get_self_ptr()
    }

    fn name(&self) -> &str {
        self.base().name()
    }

    fn referent(&self) -> Ref {
        self.base().referent()
    }

    fn remove_child(&mut self, referent: Ref) -> Result<(), RemoveChildError> {
        self.base_mut().remove_child(referent)
    }

    fn set_name(&mut self, name: &str) {
        self.base_mut().set_name(name);
    }

    fn set_parent(&mut self, parent: Option<Instance>) {
        self.base_mut().set_parent(parent)
    }
}

pub trait CreatableInstance {
    fn create(parent: Option<Instance>) -> Instance;
}

pub trait InstanceCastable: AnyInstance + DefaultClassName {
    fn downcast<T: AnyInstance + DefaultClassName>(obj: &dyn AnyInstance) -> Option<&T>;
    fn downcast_mut<T: AnyInstance + DefaultClassName>(obj: &mut dyn AnyInstance)
        -> Option<&mut T>;
}