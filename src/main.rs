use std::time::Instant;

use ribbon_api::instance::prelude::*;

async fn _start() {
    let arena = InstanceArena::default();

    let part = Instance::new::<Part>(None, arena.clone()).await;
    let part_1 = Instance::new::<Part>(Some(part), arena.clone()).await;

    let arena_cn = arena.read().await;

    let mut part = arena_cn[part].write().await;
    let part_1 = arena_cn[part_1].read().await;

    part.set_parent(Some(part_1.arena_id()), arena.clone())
        .await;
    println!("{:#?}", part_1.parent());
}

#[tokio::main]
async fn main() {
    _start().await;
}
