use super::Instance;
use crate::private::Sealed;

use mlua::prelude::*;

/// This allows to create Instance objects inside the Lua environment.
pub struct InstanceConstructor;

impl LuaUserData for InstanceConstructor {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "new",
            |_lua, (class, parent): (String, Option<Instance>)| {
                Instance::new_from_class(false, class.as_str(), parent).ok_or_else(|| {
                    LuaError::external(format!(
                        "'{}' is not a valid class to create Instance",
                        class
                    ))
                })
            },
        );
    }
}

pub trait InstanceLuaImpl: Sealed {
    fn _lua_get_property<'lua>(
        &self,
        _lua: &'lua Lua,
        _key: &str,
    ) -> LuaResult<Option<LuaValue<'lua>>> {
        Ok(None)
    }

    fn _lua_set_property<'lua>(
        &mut self,
        _lua: &'lua Lua,
        _key: &str,
        _value: LuaValue<'lua>,
    ) -> LuaResult<Option<()>> {
        Ok(None)
    }
}

impl LuaUserData for Instance {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Eq, |_lua, this, other: Instance| {
            Ok(this.ptr == other.ptr)
        });

        methods.add_meta_method_mut(
            LuaMetaMethod::NewIndex,
            |lua, this, (key, value): (String, LuaValue<'lua>)| {
                let key = key.as_str();
                let this = this.get_mut();
                match key {
                    "Name" => match value {
                        LuaValue::String(str) => {
                            let base = this.base_mut();
                            base.name = str.to_str()?.to_owned();
                            Ok(())
                        }
                        _ => Err(mlua::Error::external(format!(
                            "'{}.Name' must be a string",
                            this.class_name()
                        ))),
                    },
                    "Parent" => {
                        let inst = Instance::from_lua(value, lua)?;
                        this.set_parent(inst);
                        Ok(())
                    }
                    _ => match this._lua_set_property(lua, key, value)? {
                        Some(..) => Ok(()),
                        None => Err(LuaError::external(format!(
                            "'{}.{}' cannot be changed",
                            this.class_name(),
                            key
                        ))),
                    },
                }
            },
        );

        methods.add_meta_method(LuaMetaMethod::Index, |lua, this, key: String| {
            let key = key.as_str();
            let this = this.get();
            match key {
                "ClassName" => this.class_name().to_lua(lua),
                "Name" => this.name().to_lua(lua),
                "Parent" => this.parent().to_lua(lua),

                "FindFirstChild" => lua
                    .create_function(
                        |lua, (inst, name, recursive): (Instance, String, Option<bool>)| {
                            let recursive = recursive.unwrap_or_default();
                            inst.get().find_first_child(&name, recursive).to_lua(lua)
                        },
                    )
                    .map(LuaValue::Function),

                "GetChildren" => lua
                    .create_function(|lua, inst: Instance| {
                        Vec::to_lua(inst.get().children().cloned().collect::<Vec<_>>(), lua)
                    })
                    .map(LuaValue::Function),

                "GetDescendants" => lua
                    .create_function(|lua, inst: Instance| {
                        Vec::to_lua(inst.get().descendants().collect::<Vec<_>>(), lua)
                    })
                    .map(LuaValue::Function),

                _ => {
                    // other properties
                    if let Some(value) = this._lua_get_property(lua, key)? {
                        return Ok(value);
                    }

                    // children
                    if let Some(child) = this.find_first_child(key, false) {
                        return child.to_lua(lua);
                    }

                    Err(LuaError::external(format!(
                        "'{}' is not a valid member of {}",
                        key,
                        this.class_name()
                    )))
                }
            }
        });
    }
}
