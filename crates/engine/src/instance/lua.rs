// use std::str::FromStr;

use super::{classes, ClassName, Instance};
use mlua::prelude::*;
use std::str::FromStr;

pub struct InstanceConstructor;

// Base instance lua impl
impl classes::BaseInstance {
    pub(crate) fn lua_get_property<'lua>(
        &self,
        lua: &'lua mlua::Lua,
        key: &str,
    ) -> mlua::Result<Option<mlua::Value<'lua>>> {
        Ok(match key {
            "Archivable" => Some(self.get_archivable().to_lua(lua)?),
            "ClassName" => Some(self.get_class_name().to_lua(lua)?),
            "Name" => Some(self.get_name().to_lua(lua)?),
            "Parent" => Some(self.get_parent().to_lua(lua)?),
            _ => return Ok(None),
        })
    }

    #[allow(unused)]
    pub(crate) fn lua_set_property<'lua>(
        &mut self,
        lua: &'lua mlua::Lua,
        key: &str,
        value: mlua::Value<'lua>,
    ) -> mlua::Result<Option<()>> {
        match key {
            "Archivable" => {
                self.set_archivable(bool::from_lua(value, lua)?);
            }
            "Name" => {
                self.set_name(String::from_lua(value, lua)?.into());
            }
            "Parent" => {
                self.set_parent(Option::<Instance>::from_lua(value, lua)?)
                    .map_err(LuaError::external)?;
            }
            _ => return Ok(None),
        };
        Ok(Some(()))
    }
}

impl Instance {
    pub(crate) fn new_from_lua(class_name: ClassName, parent: Option<Instance>) -> Option<Self> {
        Some(match class_name {
            ClassName::Clouds => Instance::new::<classes::Clouds>(parent),
            ClassName::Model => Instance::new::<classes::Model>(parent),
            _ => return None,
        })
    }
}

impl LuaUserData for InstanceConstructor {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "new",
            |_lua, (class, parent): (ClassName, Option<Instance>)| {
                Instance::new_from_lua(class, parent)
                    .ok_or_else(|| LuaError::external(format!("`{class}` is not creatable")))
            },
        );
    }
}

impl<'lua> ToLua<'lua> for ClassName {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.to_string().to_lua(lua)
    }
}

impl<'lua> FromLua<'lua> for ClassName {
    fn from_lua(lua_value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::String(n) => {
                let n = n.to_string_lossy();
                ClassName::from_str(&n)
                    .map_err(|_| LuaError::external(format!("unknown class {n:#?}")))
            }
            _ => Err(LuaError::external(format!(
                "expected `string`, got `{}`",
                lua_value.type_name()
            ))),
        }
    }
}

impl LuaUserData for Instance {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |lua, this, _: ()| {
            format!("{this:?}").to_lua(lua)
        });

        methods.add_meta_method_mut(
            LuaMetaMethod::NewIndex,
            |lua, this, (key, value): (String, LuaValue<'lua>)| {
                let key = key.as_str();
                if this.lua_set_property(lua, key, value)?.is_some() {
                    return Ok(());
                }
                Err(LuaError::external(format!("`{key}` cannot be changed")))
            },
        );

        methods.add_meta_method(LuaMetaMethod::Index, |lua, this, key: String| {
            let key = key.as_str();

            if let Some(result) = this.lua_get_property(lua, key)? {
                return Ok(result);
            }

            if let Some(child) = this.find_first_child(key, false) {
                child.to_lua(lua)
            } else {
                Err(LuaError::external(format!(
                    "`{key}` is not a valid member of {}",
                    this.get_class_name(),
                )))
            }
        });
    }
}
