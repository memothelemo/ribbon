use super::BasePart;

use crate::instance::{CreatableInstance, Instance, InstanceLuaImpl, InstanceType};
use crate::private::Sealed;

#[derive(Debug)]
pub struct Part {
    pub base: BasePart,
}

impl Part {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            base: BasePart::new("Part"),
        }
    }
}

impl CreatableInstance for Part {
    fn create(_parent: Option<Instance>) -> Instance {
        Instance::new_from_trait(Self {
            base: BasePart::new("Part"),
        })
    }
}

impl Sealed for Part {}
impl InstanceType for Part {
    fn class_name(&self) -> &str {
        "Part"
    }

    fn base(&self) -> &super::BaseInstance {
        &self.base.pv.base
    }

    fn base_mut(&mut self) -> &mut super::BaseInstance {
        &mut self.base.pv.base
    }
}

impl InstanceLuaImpl for Part {
    fn _lua_get_property<'lua>(
        &self,
        lua: &'lua mlua::Lua,
        key: &str,
    ) -> mlua::Result<Option<mlua::Value<'lua>>> {
        BasePart::_lua_get_property(&self.base, lua, key)
    }

    fn _lua_set_property<'lua>(
        &mut self,
        lua: &'lua mlua::Lua,
        key: &str,
        value: mlua::Value<'lua>,
    ) -> mlua::Result<Option<()>> {
        BasePart::_lua_set_property(&mut self.base, lua, key, value)
    }
}
