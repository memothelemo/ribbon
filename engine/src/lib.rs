#![allow(incomplete_features)]
#![feature(trait_upcasting)]

pub mod instance;
pub mod lua;
pub mod types;

pub(crate) mod private;

fn visit_instance(instance: &instance::prelude::RbxInstance) {
    fn inner(instance: &instance::prelude::RbxInstance, spaces: usize) {
        println!(
            "{}{} ({})",
            "    ".repeat(spaces),
            instance.get().class(),
            instance.get().name()
        );
        for child in instance.get().children() {
            inner(child, spaces + 1);
        }
    }
    inner(instance, 0);
}

pub fn main() -> mlua::Result<()> {
    use instance::prelude::*;
    use mlua::prelude::*;

    let root = RbxInstance::new::<DataModel>(None);
    let lua = Lua::new();

    {
        let globals = lua.globals();
        globals.set("Instance", lua::LuaInstanceCtor)?;
        globals.set("game", root)?;
    }

    lua.load(
        r#"
    local instance = Instance.new("Part")
        
    "#,
    )
    .exec()?;

    Ok(())
}
