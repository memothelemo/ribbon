use crate::instance::{ClassName, Instance};

use mlua::prelude::*;
use std::str::FromStr;

pub struct LuaInstanceCtor;

impl LuaUserData for LuaInstanceCtor {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "new",
            |_lua, (class, parent): (ClassName, Option<Instance>)| {
                // This is how you do it in a hard way... :)
                Instance::new_from_class(false, class, parent)
                    .ok_or_else(|| LuaError::external(format!("`{}` is not creatable", class)))
            },
        );
    }
}

impl<'lua> FromLua<'lua> for ClassName {
    fn from_lua(lua_value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::String(str) => str
                .to_str()
                .map_err(|e| LuaError::external(format!("utf-8 error: {}", e)))
                .and_then(|v| {
                    ClassName::from_str(v)
                        .map_err(|_| LuaError::external(format!("unknown class name: {v}")))
                }),
            _ => Err(LuaError::external("cannot convert to ClassName")),
        }
    }
}

impl<'lua> ToLua<'lua> for ClassName {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        format!("{}", self).to_lua(lua)
    }
}

impl LuaUserData for Instance {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Eq, |_lua, this, other: Instance| {
            Ok(this.as_ref().referent() == other.as_ref().referent())
        });

        methods.add_meta_method(LuaMetaMethod::Index, |lua, this, key: String| {
            let key = key.as_str();
            let this = this.as_ref();

            // other properties
            if let Some(value) = this.lua_get_property(lua, key)? {
                return Ok(value);
            }

            if let Some(child) = this.find_first_child(key, false) {
                return child.to_lua(lua);
            }

            Err(LuaError::external(format!(
                "`{}` is not a valid member of {}",
                key,
                this.class()
            )))
        });
    }
}
