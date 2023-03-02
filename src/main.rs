// use engine::instance::{classes, Instance};
// use std::time::Instant;

// use engine::instance::{classes::BaseInstance, Instance};

use engine::instance::prelude::*;

fn read_tree(inst: &Instance, indent: usize) {
    println!(
        "{}-> {} ({:?})",
        "   ".repeat(indent),
        inst.get_name(),
        inst.get_class_name()
    );
    for child in inst.get_children() {
        read_tree(child, indent + 1);
    }
}

fn main() {
    let mut instance = Instance::new::<Clouds>(None);
    instance.set_name("MyClouds".into());
    instance.set_archivable(false);

    let baro = Instance::new::<Model>(None);
    instance.set_parent(Some(baro.clone())).unwrap();

    for child in baro.get_children() {
        if let Some(n) = child.cast::<BaseInstance>() {
            println!("{n:#?}");
        }
    }

    read_tree(&baro, 0);

    engine::test()
        .map_err(|e| {
            eprintln!("{e}");
            unreachable!()
        })
        .unwrap();
}
