use crate::instance::Instance;
use crate::types::Referent;

use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct BaseInstance {
    pub(crate) ptr: OnceCell<Instance>,

    pub archivable: bool,
    pub id: Referent,
    pub name: String,
    pub children: Vec<Instance>,
    pub parent: Option<Instance>,
}

impl BaseInstance {
    pub fn new(name: &'static str) -> Self {
        Self {
            archivable: true,
            ptr: OnceCell::new(),
            id: Referent::new(),
            children: Vec::new(),
            name: name.to_string(),
            parent: None,
        }
    }

    fn get_descendants(&self, inner: &mut Vec<Instance>) {
        for child in self.children.iter() {
            inner.push(child.clone());
            child.get().base().get_descendants(inner);
        }
    }
}

impl BaseInstance {
    pub fn children(&self) -> std::slice::Iter<'_, Instance> {
        self.children.iter()
    }

    pub fn descendants(&self) -> std::vec::IntoIter<Instance> {
        // we need to get the children and its children
        let mut descendants = Vec::new();
        self.get_descendants(&mut descendants);
        descendants.into_iter()
    }

    pub fn find_first_child(&self, name: &str, recursive: bool) -> Option<Instance> {
        if recursive {
            self.descendants().find(|v| v.get().name() == name)
        } else {
            self.children().find(|v| v.get().name() == name).cloned()
        }
    }

    pub fn find_first_child_of_class(&self, class_name: &str) -> Option<Instance> {
        self.children()
            .find(|v| v.get().class_name() == class_name)
            .cloned()
    }

    pub fn id(&self) -> Referent {
        self.id
    }

    pub fn clear_parent(&mut self) {
        if let Some(mut parent) = self.parent.clone() {
            let id = self.id();
            let parent = parent.get_mut();
            let position = parent
                .children()
                .position(|v| v.get().id() == id)
                .expect("My child is gone!");

            parent.base_mut().children.remove(position);
        }
    }

    pub fn set_parent(&mut self, mut parent: Instance) {
        self.clear_parent();
        // TODO: add safety doc
        // SAFETY: ptr is initialized upon creating Instance object
        unsafe {
            self.parent = Some(parent.clone_unsafe());

            let new_parent = parent.get_mut();
            new_parent
                .base_mut()
                .children
                .push(self.ptr.get_unchecked().clone());
        }
    }
}
