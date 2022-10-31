use crate::instance::{internal::PVInstanceGetter, prelude::*, BaseInstance};

#[derive(Debug)]
pub struct PVInstance {
    pub(crate) base: BaseInstance,
    pub(crate) origin_orientation: Vector3,
    pub(crate) origin_position: Vector3,
    pub(crate) pivot_origin_orientation: Vector3,
    pub(crate) pivot_origin_position: Vector3,
}

impl PVInstance {
    pub(crate) fn new(name: &'static str) -> Self {
        Self {
            base: BaseInstance::new(name),
            origin_orientation: Vector3::default(),
            origin_position: Vector3::default(),
            pivot_origin_orientation: Vector3::default(),
            pivot_origin_position: Vector3::default(),
        }
    }

    pub(crate) fn lua_get_property<'lua>(
        &self,
        lua: &'lua mlua::Lua,
        name: &str,
    ) -> mlua::Result<Option<mlua::Value<'lua>>> {
        use mlua::ToLua;
        match name {
            "Origin Orientation" => Ok(Some(self.origin_orientation.to_lua(lua)?)),
            "Origin Position" => Ok(Some(self.origin_position.to_lua(lua)?)),
            "Pivot Origin Orientation" => Ok(Some(self.pivot_origin_orientation.to_lua(lua)?)),
            "Pivot Origin Position" => Ok(Some(self.pivot_origin_position.to_lua(lua)?)),
            _ => Ok(None),
        }
    }
}

pub trait PVInstanceType: PVInstanceGetter + InstanceType {
    fn origin_orientation(&self) -> Vector3 {
        self.pv().origin_orientation
    }

    fn set_origin_orientation(&mut self, vector: Vector3) {
        self.pv_mut().origin_orientation = vector;
    }

    fn origin_position(&self) -> Vector3 {
        self.pv().origin_position
    }

    fn set_origin_position(&mut self, vector: Vector3) {
        self.pv_mut().origin_position = vector;
    }

    fn pivot_origin_orientation(&self) -> Vector3 {
        self.pv().pivot_origin_orientation
    }

    fn set_pivot_origin_orientation(&mut self, vector: Vector3) {
        self.pv_mut().pivot_origin_orientation = vector;
    }

    fn pivot_origin_position(&self) -> Vector3 {
        self.pv().pivot_origin_position
    }

    fn set_pivot_origin_position(&mut self, vector: Vector3) {
        self.pv_mut().pivot_origin_position = vector;
    }
}
