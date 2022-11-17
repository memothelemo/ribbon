pub trait Sealed {}

pub trait InstanceLuaImpl {
    fn lua_get_property<'lua>(
        &self,
        _lua: &'lua mlua::Lua,
        _key: &str,
    ) -> mlua::Result<Option<mlua::Value<'lua>>> {
        Ok(None)
    }

    fn lua_set_property<'lua>(
        &self,
        _lua: &'lua mlua::Lua,
        _key: &str,
        _value: mlua::Value<'lua>,
    ) -> mlua::Result<Option<()>> {
        Ok(None)
    }
}

macro_rules! default_instance_lua_impl {
    ($name:ident) => {
        impl InstanceLuaImpl for $name {
            fn lua_get_property<'lua>(
                &self,
                lua: &'lua mlua::Lua,
                key: &str,
            ) -> mlua::Result<Option<mlua::Value<'lua>>> {
                self.base.lua_get_property(lua, key)
            }

            fn lua_set_property<'lua>(
                &self,
                lua: &'lua mlua::Lua,
                key: &str,
                value: mlua::Value<'lua>,
            ) -> mlua::Result<Option<()>> {
                self.base.lua_set_property(lua, key, value)
            }
        }
    };
}
pub(crate) use default_instance_lua_impl;
