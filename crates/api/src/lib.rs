#![allow(incomplete_features, trivial_bounds)]

pub mod instance;

pub fn test() {
    use instance::prelude::*;
    use mlua::Lua;

    let lua = Lua::new();
    let globals = lua.globals();
    globals
        .set("myInstance", Instance::new::<Part>(None))
        .unwrap();

    lua.load(
        r#"
    local that = myInstance.Name
    print("Lua is working! " .. that)
    "#,
    )
    .exec()
    .unwrap();
}
