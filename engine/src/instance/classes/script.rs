use super::BaseScript;

use crate::instance::{CreatableInstance, Instance, InstanceLuaImpl, InstanceType};
use crate::private::Sealed;

#[derive(Debug)]
pub struct Script {
    pub base: BaseScript,
    pub source: String,
}

impl Script {
    pub fn new() -> Self {
        Script {
            base: BaseScript::new("Script"),
            source: String::new(),
        }
    }
}

impl CreatableInstance for Script {
    fn create(_parent: Option<Instance>) -> Instance {
        Instance::new_from_trait(Self::new())
    }
}

impl Sealed for Script {}
impl InstanceType for Script {
    fn class_name(&self) -> &str {
        "Script"
    }

    fn base(&self) -> &super::BaseInstance {
        &self.base.lsc.base
    }

    fn base_mut(&mut self) -> &mut super::BaseInstance {
        &mut self.base.lsc.base
    }
}

impl InstanceLuaImpl for Script {
    fn _lua_get_property<'lua>(
        &self,
        lua: &'lua mlua::Lua,
        key: &str,
    ) -> mlua::Result<Option<mlua::Value<'lua>>> {
        use mlua::ToLua;
        Ok(Some(match key {
            "Enabled" => self.base.enabled.to_lua(lua)?,
            "Source" => self.source.as_str().to_lua(lua)?,
            _ => return Ok(None),
        }))
    }

    fn _lua_set_property<'lua>(
        &mut self,
        _lua: &'lua mlua::Lua,
        key: &str,
        value: mlua::Value<'lua>,
    ) -> mlua::Result<Option<()>> {
        Ok(match key {
            "Enabled" => match value {
                mlua::Value::Boolean(value) => {
                    self.base.enabled = value;
                    Some(())
                }
                _ => {
                    return Err(mlua::Error::external(format!(
                        "{}.Enabled must be a boolean",
                        self.class_name()
                    )))
                }
            },
            _ => None,
        })
    }
}
