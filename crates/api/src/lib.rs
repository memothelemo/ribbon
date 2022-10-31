#![allow(incomplete_features, trivial_bounds)]

pub mod instance;

pub fn test() {
    use instance::prelude::*;
    use mlua::Lua;

    let lua = Lua::new();
    let globals = lua.globals();
    globals.set("Instance", InstanceBuilder).unwrap();

    lua.load(
        r#"
    local part = Instance.new("Part")
    part.Name = "Foo"
    print(part.Name)

    local parento = Instance.new("Part", part)
    _G.parento = parento
    "#,
    )
    .exec()
    .map_err(|e| eprintln!("{e}"))
    .unwrap();

    let globals = lua.globals();
    let inst = globals.get::<&str, Instance>("parento").unwrap();
    println!("{:#?}", inst.get());

    lua.load(
        r#"
    parento.Name = "ParentoBar"
    assert(parento.Parent.Name, "Foo")"#,
    )
    .exec()
    .map_err(|e| println!("{e}"))
    .unwrap();
}
