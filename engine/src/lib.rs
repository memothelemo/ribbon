#![allow(incomplete_features)]
#![feature(trait_upcasting)]

pub mod instance;
pub mod types;

pub fn main() {
    use instance::prelude::*;
    use std::time::Instant;

    let now = Instant::now();
    let _root = RbxInstance::builder::<DataModel>(|root| {
        root.add_child(RbxInstance::builder::<Workspace>(|b| {
            for _ in 1..1_000 {
                b.add_child(RbxInstance::builder::<Part>(|b| b.set_name("Foo")));
            }
        }));
        root.add_child(RbxInstance::builder::<Workspace>(|b| {
            b.set_name("MyVirtualWorkspace");
        }))
    });
    let elapsed = now.elapsed();

    //visit_instance_tree(&root, 0);
    println!();
    println!("Creation: {elapsed:.2?}");
}
