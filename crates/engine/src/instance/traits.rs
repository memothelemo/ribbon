use super::{classes::BaseInstance, error::InstanceError, ClassName, Instance};
use crate::private::Sealed;
use std::{borrow::Cow, fmt::Debug};

pub(crate) trait NoncreatableInstance: AnyInstance {
    fn create(name: &'static str, class_name: ClassName) -> Self;
}

pub trait CreatableInstance: AnyInstance {
    fn create() -> Self;
}

pub trait DefaultClassName: Sealed {
    fn default_class_name() -> ClassName;
}

pub trait AnyInstance: Debug + Sealed {
    #[doc(hidden)]
    fn base(&self) -> &BaseInstance;
    #[doc(hidden)]
    fn base_mut(&mut self) -> &mut BaseInstance;

    #[doc(hidden)]
    fn parent(&self) -> Option<&dyn AnyInstance> {
        None
    }

    #[doc(hidden)]
    fn parent_mut(&mut self) -> Option<&mut dyn AnyInstance> {
        None
    }

    #[doc(hidden)]
    #[allow(unused)]
    fn lua_get_property<'lua>(
        &self,
        lua: &'lua mlua::Lua,
        key: &str,
    ) -> mlua::Result<Option<mlua::Value<'lua>>> {
        BaseInstance::lua_get_property(self.base(), lua, key)
    }

    #[allow(unused)]
    fn lua_set_property<'lua>(
        &mut self,
        lua: &'lua mlua::Lua,
        key: &str,
        value: mlua::Value<'lua>,
    ) -> mlua::Result<Option<()>> {
        BaseInstance::lua_set_property(self.base_mut(), lua, key, value)
    }

    fn find_first_child(&self, name: &str, recursive: bool) -> Option<Instance> {
        BaseInstance::find_first_child(self.base(), name, recursive)
    }

    fn find_first_child_which_is(
        &self,
        class_name: ClassName,
        recursive: bool,
    ) -> Option<Instance> {
        BaseInstance::find_first_child_which_is(self.base(), class_name, recursive)
    }

    fn is_a(&self, class_name: ClassName) -> bool {
        BaseInstance::is_a(self.base(), class_name)
    }

    fn get_archivable(&self) -> bool {
        BaseInstance::get_archivable(self.base())
    }

    fn get_children(&self) -> &[Instance] {
        BaseInstance::get_children(self.base())
    }

    fn get_class_name(&self) -> ClassName {
        BaseInstance::get_class_name(self.base())
    }

    fn get_name(&self) -> &str {
        BaseInstance::get_name(self.base())
    }

    fn get_parent(&self) -> Option<Instance> {
        BaseInstance::get_parent(self.base())
    }

    // setters
    fn set_archivable(&mut self, value: bool) {
        BaseInstance::set_archivable(self.base_mut(), value)
    }

    fn set_name(&mut self, name: Cow<'static, str>) {
        BaseInstance::set_name(self.base_mut(), name)
    }

    fn set_parent(&mut self, parent: Option<Instance>) -> Result<(), InstanceError> {
        BaseInstance::set_parent(self.base_mut(), parent)
    }
}
