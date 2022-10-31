use mlua::ToLua;

use super::internal::{BaseInstanceGetter, Sealed};
use super::Instance;

use crate::instance::prelude::*;

#[derive(Debug)]
pub struct BaseInstance {
    pub(crate) id: Ref,
    pub(crate) name: String,
    pub(crate) children: Vec<Instance>,
    pub(crate) parent: Option<Instance>,
}

impl BaseInstance {
    pub(crate) fn new(name: &'static str) -> Self {
        Self {
            id: Ref::new(),
            children: Vec::new(),
            name: name.to_string(),
            parent: None,
        }
    }
}

pub trait InstanceType: Sealed + BaseInstanceGetter + std::any::Any + std::fmt::Debug {
    fn _lua_get_property<'lua>(
        &self,
        _lua: &'lua mlua::Lua,
        _name: &str,
    ) -> mlua::Result<Option<mlua::Value<'lua>>> {
        Ok(None)
    }

    fn _lua_set_property<'lua>(
        &self,
        _lua: &'lua mlua::Lua,
        _name: &str,
        _value: mlua::Value<'lua>,
    ) -> mlua::Result<()> {
        Err(mlua::Error::external(format!(
            "'{}' is not a valid member of {}",
            _name,
            self.class_name()
        )))
    }

    fn _lua_meta_new_index<'lua>(
        &mut self,
        _lua: &'lua mlua::Lua,
        key: &str,
        value: mlua::Value<'lua>,
    ) -> mlua::Result<()> {
        match key {
            "ClassName" => Err(mlua::Error::external("Instance.ClassName is read-only")),
            "Name" => match value {
                mlua::Value::String(str) => {
                    self.base_mut().name = str.to_str()?.to_string();
                    Ok(())
                }
                _ => Err(mlua::Error::external("Instance.Name must be a string")),
            },
            _ => self._lua_set_property(_lua, key, value),
        }
    }

    fn _lua_meta_index<'lua>(
        &self,
        lua: &'lua mlua::Lua,
        key: &str,
    ) -> mlua::Result<mlua::Value<'lua>> {
        match key {
            "Name" => self.name().to_lua(lua),
            "ClassName" => self.class_name().to_lua(lua),
            "Parent" => self.parent().to_lua(lua),

            // Getting an unknown key falls back to properties, then children.
            _ => {
                if let Some(value) = self._lua_get_property(lua, key)? {
                    return Ok(value);
                }
                if let Some(child) = self.find_first_child(key) {
                    return child.to_lua(lua);
                }
                Err(mlua::Error::external(format!(
                    "'{}' is not a valid member of {}",
                    key,
                    self.class_name()
                )))
            }
        }
    }

    fn id(&self) -> Ref {
        self.base().id
    }

    fn children(&self) -> &[Instance] {
        &self.base().children
    }

    fn class_name(&self) -> &'static str;

    fn find_first_child(&self, name: &str) -> Option<Instance> {
        self.children()
            .iter()
            .find(|inst| inst.get().name() == name)
            .cloned()
    }

    fn name(&self) -> &str {
        &self.base().name
    }

    fn name_mut(&mut self) -> &mut String {
        &mut self.base_mut().name
    }

    fn parent(&self) -> Option<Instance> {
        self.base().parent.clone()
    }
}
