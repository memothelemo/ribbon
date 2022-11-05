use crate::instance::classes::BaseInstance;
use crate::instance::{Instance, InstanceLuaImpl};

use crate::private::Sealed;
use crate::types::Referent;

pub trait CreatableInstance {
    fn create(parent: Option<Instance>) -> Instance;
}

pub trait InstanceType: Sealed + InstanceLuaImpl {
    #[doc(hidden)]
    fn base(&self) -> &BaseInstance;

    #[doc(hidden)]
    fn base_mut(&mut self) -> &mut BaseInstance;

    /// Determines if an [Instance] can be cloned using
    /// [`Instance::Clone`] or saved to file.
    ///
    /// [Instance]: crate::instance::Instance
    /// [`Instance::Clone`]: crate::instance::InstanceType::clone
    fn archivable(&self) -> bool {
        self.base().archivable
    }

    /// A read-only string representing the class
    /// this [Instance] belongs to.
    ///
    /// [Instance]: crate::instance::Instance
    fn class_name(&self) -> &str;

    fn children(&self) -> std::slice::Iter<'_, Instance> {
        self.base().children()
    }

    fn descendants(&self) -> std::vec::IntoIter<Instance> {
        self.base().descendants()
    }

    fn find_first_child(&self, name: &str, recursive: bool) -> Option<Instance> {
        self.base().find_first_child(name, recursive)
    }

    fn find_first_child_of_class(&self, class_name: &str) -> Option<Instance> {
        self.base().find_first_child_of_class(class_name)
    }

    fn id(&self) -> Referent {
        self.base().id
    }

    /// A non-unique identifier of the [Instance].
    ///
    /// [Instance]: crate::instance::Instance
    fn name(&self) -> &str {
        &self.base().name
    }

    fn set_name(&mut self, name: String) {
        self.base_mut().name = name;
    }

    /// Determines the hierarchical parent of the [Instance].
    ///
    /// [Instance]: crate::instance::Instance
    fn parent(&self) -> Option<Instance> {
        self.base().parent.clone()
    }

    fn clear_parent(&mut self) {
        self.base_mut().clear_parent();
    }

    fn set_parent(&mut self, parent: Instance) {
        self.base_mut().set_parent(parent);
    }
}
