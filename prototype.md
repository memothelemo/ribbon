```rs
use ribbon::prelude::*;

struct TokioWrapper;

impl RibbonLibrary for TokioWrapper {
    ...
}

#[tokio::main]
async fn main() -> Result<()> {
    let engine = Engine::builder()
        .with_parallelism(true)
        .with_networking(true)
        .stop_if_no_busy_threads(false)
        .stop_on_error(true)
        .non_wait_loop_timeout(Duration::from_secs(5))
        .add_library(TokioWrapper)
        .build()
        .unwrap();

    let script = Script::new_from_code(r#"
        -- It supports Luau from mlua
        tokio.spawn(function()
            print("Hello from Luau")
        end)
    "#);

    // or this
    let script_from_inst = Instance::new::<Script>(None);
    script_from_inst
        .cast_mut::<Script>()
        .unwrap()
        .set_source("while true do end");

    // Totally async, it also allows to run scripts
    // with strings as well, if you like it to...
    engine.run_code(script);
    engine.run_code(r#"
        tokio.sleep(tokio.Duration.fromSecs(10))
        
        local ribbon = game:GetService("RibbonManager")
        ribbon:StopEngine()
    "#);

    // while true do loops are also not supported obviously!
    engine.run_code(script_from_inst);

    // It will not going to stop indefinitely...
    engine.wait().await?;

    Ok(())
}
```

Basic:
```rs
use ribbon::prelude::*;

fn read_children(inst: &Instance) {
    for child in inst.get_children() {
        // Oh look! You can actually get access to
        // the contents of a subclass!
        if let Some(mut part) = child
            .clone()
            .cast_mut::<BasePart>()
        {
            part.set_transparency(1.);
        } else {
            println!("{}", child.get_name());
        }
    }
}

fn main() {
    // Implemented std::ops::Deref and mut so you don't have to type
    // `.any()` multiple times which it makes repeating code
    let mut instance = Instance::new::<Model>(None);
    instance.set_name("My Object");

    // We're not duplicating instance object to mak
    // new again, we're copying its pointer. Pointer is
    // like an address of your home and inside your home
    // is the actual contents of Instance object.
    //
    // TDLR: Instance behaves like std::sync::Arc
    //       with actual unknown instance object inside
    let part = Instance::new::<Part>(Some(instance.clone()));
    part.set_name("Hologram");
    part.set_material(Enum::Material::Neon);
    part.set_transparency(0.5);    // 

    // If you're actually want to perform `:Clone()` just like
    // in Roblox, you can call `.copy()` instead.
    let mut copy = instance.copy();
    copy.set_parent(Some(instance.clone()));
    read_children(&instance);
}

```