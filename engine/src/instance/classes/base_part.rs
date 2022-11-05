use super::PVInstance;

use crate::instance::InstanceLuaImpl;
use crate::types::Vector3;

#[derive(Debug)]
pub struct BasePart {
    pub pv: PVInstance,
    pub position: Vector3,
}

impl BasePart {
    pub fn new(name: &'static str) -> Self {
        Self {
            pv: PVInstance::new(name),
            position: Vector3::new(0., 0., 0.),
        }
    }
}

impl crate::private::Sealed for BasePart {}
impl InstanceLuaImpl for BasePart {
    fn _lua_get_property<'lua>(
        &self,
        lua: &'lua mlua::Lua,
        key: &str,
    ) -> mlua::Result<Option<mlua::Value<'lua>>> {
        use mlua::ToLua;
        match key {
            "Position" => self.position.to_lua(lua).map(Some),
            _ => PVInstance::_lua_get_property(&self.pv, lua, key),
        }
    }

    fn _lua_set_property<'lua>(
        &mut self,
        lua: &'lua mlua::Lua,
        key: &str,
        value: mlua::Value<'lua>,
    ) -> mlua::Result<Option<()>> {
        use mlua::FromLua;
        match key {
            "Position" => {
                let value = Vector3::from_lua(value, lua)?;
                self.position = value;
            }
            _ => return PVInstance::_lua_set_property(&mut self.pv, lua, key, value),
        }
        Ok(Some(()))
    }
}
