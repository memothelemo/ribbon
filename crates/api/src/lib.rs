#![allow(incomplete_features, trivial_bounds)]

use std::time::Instant;

pub mod ctors;
pub mod instance;

pub fn test() {
    use mlua::Lua;

    let lua = Lua::new();
    let globals = lua.globals();
    globals.set("Instance", ctors::InstanceBuilder).unwrap();
    globals.set("Vector3", ctors::Vector3Builder).unwrap();

    let binding = std::fs::read_to_string("test/my_script.lua").unwrap();
    let chunk = lua.load(&binding).set_name("test/my_script.lua").unwrap();

    let now = Instant::now();
    chunk.exec().map_err(|e| eprintln!("{e}")).unwrap();

    let elapsed = now.elapsed();
    println!("Took {:.2?}", elapsed);
}
