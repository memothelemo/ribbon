use super::prelude::*;

use once_cell::sync::OnceCell;
use std::borrow::Cow;

#[derive(Debug)]
pub struct BaseInstance {
    inner: OnceCell<Instance>,
    archivable: bool,
    class_name: ClassName,
    children: Vec<Instance>,
    name: Cow<'static, str>,
    parent: Option<Instance>,
}

impl DefaultClassName for BaseInstance {
    fn default_class_name() -> ClassName {
        ClassName::BaseInstance
    }
}

impl Sealed for BaseInstance {}
impl AnyInstance for BaseInstance {
    fn base(&self) -> &BaseInstance {
        self
    }

    fn base_mut(&mut self) -> &mut BaseInstance {
        self
    }
}

impl NoncreatableInstance for BaseInstance {
    fn create(name: &'static str, class_name: ClassName) -> Self {
        Self {
            inner: OnceCell::new(),
            archivable: true,
            class_name,
            children: Vec::new(),
            name: name.into(),
            parent: None,
        }
    }
}

// BaseInstance functions
impl BaseInstance {
    pub fn clear_all_children(&mut self) {
        // TODO: Make destroy function
    }

    pub fn find_first_child(&self, name: &str, recursive: bool) -> Option<Instance> {
        for child in self.children.iter() {
            if child.any().get_name() == name {
                return Some(child.clone());
            }
            if recursive {
                if let Some(child) = child.any().find_first_child(name, recursive) {
                    return Some(child);
                }
            }
        }
        None
    }

    pub fn find_first_child_which_is(
        &self,
        class_name: ClassName,
        recursive: bool,
    ) -> Option<Instance> {
        for child in self.children.iter() {
            if child.any().is_a(class_name) {
                return Some(child.clone());
            }
            if recursive {
                if let Some(child) = child.any().find_first_child_which_is(class_name, recursive) {
                    return Some(child);
                }
            }
        }
        None
    }

    pub fn is_a(&self, class_name: ClassName) -> bool {
        for subclass in self.class_name.base_classes() {
            if subclass.eq(&class_name) {
                return true;
            }
        }
        false
    }
}

// Property getters
impl BaseInstance {
    pub fn get_archivable(&self) -> bool {
        self.archivable
    }

    pub fn get_children(&self) -> &[Instance] {
        &self.children
    }

    pub fn get_class_name(&self) -> ClassName {
        self.class_name
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_parent(&self) -> Option<Instance> {
        self.parent.clone()
    }
}

// Property setters
impl BaseInstance {
    pub fn set_archivable(&mut self, value: bool) {
        self.archivable = value;
    }

    pub fn set_name(&mut self, name: Cow<'static, str>) {
        self.name = name;
    }

    pub fn set_parent(&mut self, parent: Option<Instance>) -> Result<(), InstanceError> {
        if parent
            .as_ref()
            .zip(self.parent.as_ref())
            .map(|(a, b)| a == b)
            .unwrap_or(false)
        {
            // No effect :)
            return Ok(());
        }

        if self
            .inner
            .get()
            .zip(parent.as_ref())
            .map(|(a, b)| a == b)
            .unwrap_or(false)
        {
            return Err(InstanceError::ParentAssignedItself);
        }

        if let Some(mut parent) = self.parent.clone() {
            let inner = self.inner.get().unwrap();
            let parent = parent.any_mut().base_mut();
            let position = parent.children.iter().position(|v| v.eq(inner)).unwrap();
            parent.children.remove(position);
        }
        self.parent = None;

        if let Some(mut parent) = parent {
            unsafe {
                self.parent = Some(parent.weak_clone());
            }
            let new_parent = parent.any_mut();
            new_parent
                .base_mut()
                .children
                .push(self.inner.get().unwrap().clone());
        }

        Ok(())
    }
}

pub(crate) unsafe fn set_inner_ptr(inst: &mut dyn AnyInstance, ptr: Instance) {
    inst.base_mut().inner.set(ptr).unwrap();
}

#[allow(unused)]
pub(crate) unsafe fn wipe_inner_ptr(inst: &mut dyn AnyInstance) {
    inst.base_mut().inner.take().unwrap();
}

pub(crate) unsafe fn drop_children(inst: &mut dyn AnyInstance) {
    for mut child in inst.base_mut().children.drain(..) {
        child.any_mut().base_mut().parent = None;
        drop(child);
    }
}
