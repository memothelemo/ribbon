pub mod instance;

pub(crate) mod private;

pub fn test() -> mlua::Result<()> {
    let lua = mlua::Lua::new();
    lua.globals()
        .set("Instance", instance::InstanceConstructor)?;

    Ok(())
}
