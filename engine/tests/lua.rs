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
                .load(
                    r#"return function(base, index, value)
                    base[index] = value
                    assert_eq(base[index], value)
                end"#,
                )
                .eval::<LuaFunction>()?;
            globals.set("assert_property", assert_property)?;

            let deep_equals = lua
                .load(
                    r#"
local function deepEqual(a: any, b: any)
    if typeof(a) ~= typeof(b) then
        return false
    end

    if typeof(a) == "table" then
        local visitedKeys = {}

        for key, value in pairs(a) do
            visitedKeys[key] = true

            local success = deepEqual(value, b[key])
            if not success then
                return false
            end
        end

        for key, value in pairs(b) do
            if not visitedKeys[key] then
                local success = deepEqual(value, a[key])
                if not success then
                    return false
                end
            end
        end

        return true
    end

    if a == b then
        return true
    end

    return false
end

return deepEqual"#,
                )
                .eval::<LuaFunction>()?;
            globals.set("deepEquals", deep_equals)?;

            let create_instance_by_tree = lua
                .load(
                    r#"
                return function(tree)
                    local function createInner(tree)
                        local instance = Instance.new(tree.ClassName)
                        for index, value in pairs(tree) do
                            if index ~= "ClassName" and index ~= "Children" then
                                instance[index] = value
                            end
                        end
                        for name, inner in pairs(tree.Children or {}) do
                            inner.Name = name
                            inner.Parent = instance
                            createInner(inner)
                        end
                        return instance
                    end
                    return createInner(tree)
                end"#,
                )
                .eval::<LuaFunction>()?;
            globals.set("createInstanceTree", create_instance_by_tree)?;

            drop(globals);
            Ok(lua)
        }
        inner().unwrap()
    }
}
