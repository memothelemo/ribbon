use rbx_types::Vector3;

use crate::instance::{prelude, Instance};

pub struct InstanceBuilder;

impl mlua::UserData for InstanceBuilder {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "new",
            |_lua, (class_name, parent): (String, Option<Instance>)| match class_name.as_str() {
                "Part" => Ok(Instance::new::<prelude::Part>(parent)),
                _ => Err(mlua::Error::external(format!(
                    "'{}' is not a valid class of Instance",
                    class_name
                ))),
            },
        );
    }
}

pub struct Vector3Builder;

impl mlua::UserData for Vector3Builder {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_function_get("zero", |_lua, _this| Ok(Vector3::default()));
        fields.add_field_function_get("one", |_lua, _this| Ok(Vector3::new(1., 1., 1.)));
        fields.add_field_function_get("xAxis", |_lua, _this| Ok(Vector3::new(1., 0., 0.)));
        fields.add_field_function_get("yAxis", |_lua, _this| Ok(Vector3::new(0., 1., 0.)));
        fields.add_field_function_get("zAxis", |_lua, _this| Ok(Vector3::new(0., 0., 1.)));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "new",
            |_lua, (x, y, z): (Option<f32>, Option<f32>, Option<f32>)| {
                let x = x.unwrap_or(0.);
                let y = y.unwrap_or(0.);
                let z = z.unwrap_or(0.);
                Ok(Vector3::new(x, y, z))
            },
        );
    }
}
