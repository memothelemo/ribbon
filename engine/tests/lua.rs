mod types;

pub mod prelude {
    pub use mlua::prelude::*;
    pub use ribbon_engine::instance::prelude::*;
    pub use ribbon_engine::types::prelude::*;

    macro_rules! test_code {
        ($name:ident, $path:literal) => {
            #[test]
            fn $name() {
                use crate::prelude::*;

                let code = include_str!($path);
                let lua = setup_lua_env();
                lua.load(code)
                    .exec()
                    .map_err(|e| eprintln!("{}", e))
                    .unwrap();
            }
        };
    }
    pub(crate) use test_code;

    pub fn setup_lua_env() -> Lua {
        fn inner() -> LuaResult<Lua> {
            let lua = Lua::new();
            let globals = lua.globals();
            globals.set("Vector3", Vector3Constructor)?;
            globals.set("Instance", RbxInstanceConstructor)?;

            // useful utilities
            let assert_eq = lua.create_function(|_, (left, right): (LuaValue, LuaValue)| {
                if left.equals(right)? {
                    Ok(())
                } else {
                    Err(LuaError::external("not the same"))
                }
            })?;
            globals.set("assert_eq", assert_eq)?;

            let assert_property = lua
                .load(include_str!("lua/assertProperty.lua"))
                .eval::<LuaFunction>()?;
            globals.set("assert_property", assert_property)?;

            let deep_equals = lua
                .load(include_str!("lua/deepEquals.lua"))
                .eval::<LuaFunction>()?;
            globals.set("deepEquals", deep_equals)?;

            let create_instance_by_tree = lua
                .load(include_str!("lua/createInstanceTree.lua"))
                .eval::<LuaFunction>()?;
            globals.set("createInstanceTree", create_instance_by_tree)?;

            drop(globals);
            Ok(lua)
        }
        inner().unwrap()
    }
}
