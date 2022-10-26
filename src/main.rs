use ribbon_api::instance::prelude::*;

fn main() {
    // Run `bash memchk.sh` to check for memory leaks, linux only
    let mut instance = Instance::new::<Part>(None);
    let mut instance_2 = Instance::new::<Part>(None);
    let mut instance_3 = Instance::new::<Part>(None);
    let mut instance_4 = Instance::new::<Part>(None);
    let mut instance_5 = Instance::new::<Part>(None);
    unsafe {
        dbg!(instance.ptr(), instance_2.ptr());
        dbg!(instance_3.ptr(), instance_4.ptr());
        dbg!(instance_5.ptr());
    }

    Instance::set_parent(&mut instance, &mut instance_4);
    Instance::set_parent(&mut instance_3, &mut instance);
    Instance::set_parent(&mut instance_4, &mut instance_2);
    Instance::set_parent(&mut instance_5, &mut instance_3);

    dbg!(instance.get().base());
    dbg!(instance_2.get().base());
    dbg!(instance_3.get().base());
    dbg!(instance_4.get().base());
    dbg!(instance_5.get().base());
}
// use std::time::Instant;

// use ribbon_api::instance::prelude::*;

// async fn _start() {
//     let arena = InstanceArena::default();

//     let part = Instance::new::<Part>(None, arena.clone()).await;
//     let part_1 = Instance::new::<Part>(Some(part), arena.clone()).await;

//     let arena_cn = arena.read().await;

//     let mut part = arena_cn[part].write().await;
//     let part_1 = arena_cn[part_1].read().await;

//     part.set_parent(Some(part_1.arena_id()), arena.clone())
//         .await;
//     println!("{:#?}", part_1.parent());
// }

// #[tokio::main]
// async fn main() {
//     _start().await;
// }
