use std::time::Instant;

use ribbon_api::instance::prelude::*;

async fn _start() {
    let now = Instant::now();
    let arena = InstanceArena::default();
    let instance = Instance::new::<Part>(None, arena.clone()).await;
    let instance_2 = Instance::new::<Part>(Some(instance), arena.clone()).await;

    get_obj!(mut instance: arena, instance);
    instance.set_parent(instance_2, arena.clone()).await;
    instance.destroy(arena.clone()).await;

    let elapsed = now.elapsed();
    println!("Took {:.2?}", elapsed);
}

#[tokio::main]
async fn main() {
    _start().await;
}
