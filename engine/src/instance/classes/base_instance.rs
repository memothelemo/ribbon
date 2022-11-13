use crate::types::Ref;
use once_cell::unsync::OnceCell;
use std::collections::VecDeque;

use super::prelude::*;

#[derive(Debug)]
pub struct BaseInstance {
    pub(crate) ptr: OnceCell<Instance>,

    referent: Ref,
    children: Vec<Instance>,
    parent: Option<Instance>,

    class: ClassName,
    name: String,
}

impl BaseInstance {
    pub(crate) fn new(name: &'static str, class: ClassName) -> Self {
        Self {
            ptr: OnceCell::new(),

            referent: Ref::new(),
            children: Vec::new(),
            parent: None,

            class,
            name: name.to_string(),
        }
    }
}

impl BaseInstance {
    fn find_child(&self, child: Ref) -> Option<usize> {
        self.children
            .iter()
            .position(|v| v.get().referent() == child)
    }
}

impl BaseInstance {
    pub fn parent(&self) -> Option<Instance> {
        self.parent.clone()
    }

    pub fn class(&self) -> ClassName {
        self.class
    }

    pub fn referent(&self) -> Ref {
        self.referent
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn children(&self) -> std::slice::Iter<'_, Instance> {
        self.children.iter()
    }

    pub fn get_self_ptr(&self) -> Instance {
        self.ptr.get().unwrap().clone()
    }

    pub fn descendants(&self) -> Vec<Instance> {
        let mut stack = VecDeque::new();
        let mut descendants = Vec::new();
        let mut current = Some(self.get_self_ptr());

        while let Some(reference) = current {
            let children = reference.get().children();
            for child in children {
                descendants.push(child.clone());
                stack.push_front(child.clone());
            }

            current = stack.pop_front();
        }

        descendants
    }
}

impl BaseInstance {
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl BaseInstance {
    pub fn add_child(&mut self, mut child: Instance) {
        child.get_mut().set_parent(Some(self.get_self_ptr()))
    }

    pub fn remove_child(&mut self, referent: Ref) -> Result<(), RemoveChildError> {
        let position = self
            .find_child(referent)
            .ok_or(RemoveChildError::NotAChild)?;

        self.children.remove(position);
        Ok(())
    }

    pub fn clear_parent(&mut self) {
        if let Some(mut parent) = self.base_mut().parent.take() {
            let parent = parent.get_mut();
            parent.remove_child(self.referent).unwrap();
        }
    }

    pub fn set_parent(&mut self, parent: Option<Instance>) {
        if let Some(mut parent) = parent {
            // not setting parent to its own object
            if let Some(old_parent) = self.parent.clone() {
                if parent.get().referent() == old_parent.get().referent() {
                    return;
                }
            }

            // not setting parent if it's already a child of it
            if self.find_child(self.referent).is_some() {
                return;
            }

            self.clear_parent();
            unsafe {
                self.parent = Some(parent.clone_no_ref());

                let new_parent = parent.get_mut();
                new_parent.base_mut().children.push(self.get_self_ptr());
            }
        } else {
            self.clear_parent();
        }
    }
}

impl BaseInstance {
    pub(crate) unsafe fn drop_children(&mut self) {
        for child in self.children.drain(..) {
            drop(child);
        }
    }
}

// SAFETY: BaseInstance as it is, you cannot move it down further
impl InstanceCastable for BaseInstance {
    fn downcast<T: AnyInstance + DefaultClassName>(obj: &dyn AnyInstance) -> Option<&T> {
        use crate::instance::utils::unsafe_transmute;
        unsafe {
            if T::default_class_name() == ClassName::BaseInstance {
                Some(unsafe_transmute::<T>(obj))
            } else {
                None
            }
        }
    }

    fn downcast_mut<T: AnyInstance + DefaultClassName>(
        obj: &mut dyn AnyInstance,
    ) -> Option<&mut T> {
        use crate::instance::utils::unsafe_transmute_mut;
        unsafe {
            if T::default_class_name() == ClassName::BaseInstance {
                Some(unsafe_transmute_mut::<T>(obj))
            } else {
                None
            }
        }
    }
}

impl DefaultClassName for BaseInstance {
    fn default_class_name() -> ClassName {
        ClassName::BaseInstance
    }
}

impl AnyInstance for BaseInstance {
    fn base(&self) -> &BaseInstance {
        self
    }

    fn base_mut(&mut self) -> &mut BaseInstance {
        self
    }
}
