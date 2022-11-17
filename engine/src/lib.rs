#![allow(incomplete_features)]
#![feature(trait_upcasting)]

pub mod instance;
pub mod types;

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

pub fn main() {
    use instance::prelude::*;
    use std::time::Instant;

    let now = Instant::now();
    let root = RbxInstance::builder::<DataModel>(|root| {
        let workspace = root.workspace_mut();
        for _ in 1..10 {
            workspace.add_child(RbxInstance::builder::<Part>(|b| b.set_name("Foo")));
        }
        root.add_child(RbxInstance::builder::<Workspace>(|b| {
            b.set_name("MyVirtualWorkspace");
        }));
    });

    let elapsed = now.elapsed();

    println!(
        "{:#?}",
        root.cast::<ServiceProvider>()
            .unwrap()
            .get_service(RbxClassName::RibbonManager)
            .unwrap()
            .cast::<RibbonManager>()
            .unwrap()
    );
    visit_instance(&root);
    println!();
    println!("Creation: {elapsed:.2?}");
}
