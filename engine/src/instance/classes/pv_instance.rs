use super::BaseInstance;

use crate::instance::InstanceLuaImpl;
use crate::types::Vector3;

#[derive(Debug)]
pub struct PVInstance {
    pub base: BaseInstance,
    pub origin_orientation: Vector3,
    pub origin_position: Vector3,
    pub pivot_origin_orientation: Vector3,
    pub pivot_origin_position: Vector3,
}

impl PVInstance {
    pub fn new(name: &'static str) -> Self {
        Self {
            base: BaseInstance::new(name),
            origin_orientation: Vector3::default(),
            origin_position: Vector3::default(),
            pivot_origin_orientation: Vector3::default(),
            pivot_origin_position: Vector3::default(),
        }
    }
}

impl crate::private::Sealed for PVInstance {}
impl InstanceLuaImpl for PVInstance {
    fn _lua_get_property<'lua>(
        &self,
        lua: &'lua mlua::Lua,
        key: &str,
    ) -> mlua::Result<Option<mlua::Value<'lua>>> {
        use mlua::ToLua;
        match key {
            "Origin Orientation" => self.origin_orientation.to_lua(lua).map(Some),
            "Origin Position" => self.origin_position.to_lua(lua).map(Some),
            "Pivot Origin Orientation" => self.pivot_origin_orientation.to_lua(lua).map(Some),
            "Pivot Origin Position" => self.pivot_origin_position.to_lua(lua).map(Some),
            _ => Ok(None),
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
            "Origin Orientation" => {
                let value = Vector3::from_lua(value, lua)?;
                self.origin_orientation = value;
            }
            "Origin Position" => {
                let value = Vector3::from_lua(value, lua)?;
                self.origin_position = value;
            }
            "Pivot Origin Orientation" => {
                let value = Vector3::from_lua(value, lua)?;
                self.pivot_origin_orientation = value;
            }
            "Pivot Origin Position" => {
                let value = Vector3::from_lua(value, lua)?;
                self.pivot_origin_position = value;
            }
            _ => return Ok(None),
        }
        Ok(Some(()))
    }
}
