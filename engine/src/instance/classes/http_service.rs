use super::BaseInstance;
use uuid::Uuid;

use crate::instance::{CreatableInstance, Instance, InstanceLuaImpl, InstanceType};
use crate::private::Sealed;

#[derive(Debug)]
pub struct HttpService {
    pub base: BaseInstance,
    pub http_enabled: bool,
}

impl HttpService {
    pub fn generate_guid(wrap_in_curly_braces: bool) -> String {
        let uuid = Uuid::new_v4().to_string();
        if wrap_in_curly_braces {
            format!("{{{uuid}}}")
        } else {
            uuid
        }
    }

    pub fn get_http_enabled(&self) -> bool {
        self.http_enabled
    }

    pub fn get_user_agent(&self) -> &'static str {
        concat!(
            "ribbon (https://github.com/memothelemo/ribbon) ",
            env!("CARGO_PKG_VERSION")
        )
    }

    pub fn json_decode(input: &str) -> serde_json::Result<serde_json::Value> {
        serde_json::from_str(input)
    }

    pub fn json_encode(input: &serde_json::Value) -> serde_json::Result<String> {
        serde_json::to_string(&input)
    }
}

impl CreatableInstance for HttpService {
    fn create(_parent: Option<Instance>) -> Instance {
        Instance::new_from_trait(Self {
            base: BaseInstance::new("HttpService"),
            http_enabled: true,
        })
    }
}

impl Sealed for HttpService {}
impl InstanceType for HttpService {
    fn class_name(&self) -> &str {
        "HttpService"
    }

    fn base(&self) -> &BaseInstance {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseInstance {
        &mut self.base
    }
}

impl InstanceLuaImpl for HttpService {
    fn _lua_get_property<'lua>(
        &self,
        lua: &'lua mlua::Lua,
        key: &str,
    ) -> mlua::Result<Option<mlua::Value<'lua>>> {
        use mlua::ToLua;
        match key {
            "HttpEnabled" => self.http_enabled.to_lua(lua).map(Some),

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
            "HttpEnabled" => {
                self.http_enabled = bool::from_lua(value, lua)?;
            }
            _ => return Ok(None),
        }
        Ok(Some(()))
    }
}
