use mlua::prelude::*;
use std::marker::PhantomData;

use super::*;

enum Either<'lua, L, R> {
    Left(L, PhantomData<&'lua Lua>),
    Right(R, PhantomData<&'lua Lua>),
}

impl<'lua, L, R> mlua::FromLua<'lua> for Either<'lua, L, R>
where
    L: mlua::FromLua<'lua>,
    R: mlua::FromLua<'lua>,
{
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let another = value.clone();
        if let Ok(value) = L::from_lua(value, lua) {
            return Ok(Either::Left(value, PhantomData));
        }
        if let Ok(value) = R::from_lua(another, lua) {
            return Ok(Either::Right(value, PhantomData));
        }
        Err(LuaError::external("invalid type"))
    }
}

impl<'lua, L, R> mlua::ToLua<'lua> for Either<'lua, L, R>
where
    L: mlua::ToLua<'lua>,
    R: mlua::ToLua<'lua>,
{
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        match self {
            Self::Left(n, ..) => n.to_lua(lua),
            Self::Right(n, ..) => n.to_lua(lua),
        }
    }
}

/// This allows to create Vector3 objects inside the Lua environment.
pub struct Vector3Constructor;

impl LuaUserData for Vector3Constructor {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_function_get("one", |_lua, _| Ok(Vector3::new(1., 1., 1.)));
        fields.add_field_function_get("xAxis", |_lua, _| Ok(Vector3::new(1., 0., 0.)));
        fields.add_field_function_get("yAxis", |_lua, _| Ok(Vector3::new(0., 1., 0.)));
        fields.add_field_function_get("zAxis", |_lua, _| Ok(Vector3::new(0., 0., 1.)));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "new",
            |_lua, (x, y, z): (Option<f64>, Option<f64>, Option<f64>)| {
                Ok(Vector3::new(
                    x.unwrap_or_default(),
                    y.unwrap_or_default(),
                    z.unwrap_or_default(),
                ))
            },
        );
    }
}

impl LuaUserData for Vector3 {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Index, |lua, this, key: String| {
            match key.to_lowercase().as_str() {
                "x" => this.x.to_lua(lua),
                "y" => this.y.to_lua(lua),
                "z" => this.z.to_lua(lua),
                "magnitude" => {
                    f64::sqrt(this.x.powi(2) + this.y.powi(2) + this.z.powi(2)).to_lua(lua)
                }
                v => Err(LuaError::external(format!(
                    "'{}' is not a valid member of Vector3",
                    v
                ))),
            }
        });

        methods.add_meta_method_mut(
            LuaMetaMethod::NewIndex,
            |lua, this, (key, value): (String, LuaValue)| {
                match key.to_lowercase().as_str() {
                    "x" => {
                        let value = f64::from_lua(value, lua)?;
                        this.x = value;
                    }
                    "y" => {
                        let value = f64::from_lua(value, lua)?;
                        this.y = value;
                    }
                    "z" => {
                        let value = f64::from_lua(value, lua)?;
                        this.z = value;
                    }
                    v => return Err(LuaError::external(format!("'{}' cannot be changed", v))),
                }
                Ok(())
            },
        );

        methods.add_meta_method(LuaMetaMethod::Add, |_lua, left, right: Vector3| {
            Ok(Self {
                x: left.x + right.x,
                y: left.y + right.y,
                z: left.z + right.z,
            })
        });

        methods.add_meta_method(LuaMetaMethod::Sub, |_lua, left, right: Vector3| {
            Ok(Self {
                x: left.x - right.x,
                y: left.y - right.y,
                z: left.z - right.z,
            })
        });

        methods.add_meta_method(
            LuaMetaMethod::Mul,
            |_lua, left, right: Either<'lua, Vector3, f64>| {
                Ok(match right {
                    Either::Left(vector, ..) => Self {
                        x: left.x * vector.x,
                        y: left.y * vector.y,
                        z: left.z * vector.z,
                    },
                    Either::Right(multipler, ..) => Self {
                        x: left.x * multipler,
                        y: left.y * multipler,
                        z: left.z * multipler,
                    },
                })
            },
        );

        methods.add_meta_method(
            LuaMetaMethod::Div,
            |_lua, left, right: Either<'lua, Vector3, f64>| {
                Ok(match right {
                    Either::Left(vector, ..) => Self {
                        x: left.x / vector.x,
                        y: left.y / vector.y,
                        z: left.z / vector.z,
                    },
                    Either::Right(divisor, ..) => Self {
                        x: left.x / divisor,
                        y: left.y / divisor,
                        z: left.z / divisor,
                    },
                })
            },
        );

        methods.add_meta_method(LuaMetaMethod::Eq, |_lua, left, right: Vector3| {
            // Vector3 implements PartialEq
            Ok(left.eq(&right))
        });
    }
}
