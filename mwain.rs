use std::{
    any::Any,
    mem::MaybeUninit,
    sync::{Arc, Mutex},
};

use id_arena::{Arena, Id};

type InstanceRef = Arc<Mutex<dyn Instance + 'static>>;
type InstanceArena = Arena<InstanceRef>;

#[derive(Debug)]
struct BaseInstance {
    id: MaybeUninit<Id<InstanceRef>>,
    name: String,
    children: Vec<Id<InstanceRef>>,
    parent: Option<Id<InstanceRef>>,
}

impl Instance for BaseInstance {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_base(&self) -> &BaseInstance {
        self
    }

    fn get_mut_base(&mut self) -> &mut BaseInstance {
        self
    }
}

trait Instance: std::any::Any + 'static {
    fn as_any(&self) -> &dyn Any;

    fn get_base(&self) -> &BaseInstance;
    fn get_mut_base(&mut self) -> &mut BaseInstance;

    fn name(&self) -> &str {
        &self.get_base().name
    }

    fn name_mut(&mut self) -> &mut str {
        &mut self.get_mut_base().name
    }

    fn set_parent(&mut self, arena: &InstanceArena, id: Id<InstanceRef>) {
        unsafe {
            let base = self.get_mut_base();
            let self_id = base.id.assume_init();
            arena[id]
                .lock()
                .unwrap()
                .get_mut_base()
                .children
                .push(self_id);
            base.parent = Some(id);
        }
    }
}

impl std::fmt::Debug for dyn Instance + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            f.debug_tuple("Instance")
                .field(&self.get_base().id.assume_init())
                .finish()
        }
    }
}

struct BasePartImpl {
    base: BaseInstance,
    position: usize,
}

impl Instance for BasePartImpl {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_base(&self) -> &BaseInstance {
        &self.base
    }

    fn get_mut_base(&mut self) -> &mut BaseInstance {
        &mut self.base
    }
}

trait BasePart: Instance {
    fn r#impl(&self) -> &BasePartImpl;
    fn r#impl_mut(&mut self) -> &mut BasePartImpl;

    fn position(&self) -> usize {
        self.r#impl().position
    }

    fn position_mut(&mut self) -> &mut usize {
        &mut self.r#impl_mut().position
    }
}

fn main() {
    let mut arena: InstanceArena = Arena::new();
    let too = arena.alloc(Arc::new(Mutex::new(BaseInstance {
        id: MaybeUninit::uninit(),
        name: "too".into(),
        children: Default::default(),
        parent: None,
    })));
    arena
        .get(too)
        .unwrap()
        .lock()
        .unwrap()
        .get_mut_base()
        .id
        .write(too);

    let too2 = arena.alloc(Arc::new(Mutex::new(BaseInstance {
        id: MaybeUninit::uninit(),
        name: "too2".into(),
        children: Default::default(),
        parent: None,
    })));
    let mut instance = arena[too2].lock().unwrap();
    instance.set_parent(&arena, too);

    match instance.as_any().downcast_ref::<BasePartImpl>() {
        Some(n) => unsafe {
            println!("{:#?}", <dyn Instance>::get_base(n).id.assume_init());
        },
        None => todo!(),
    }

    drop(instance);
    println!("{:#?}", arena[too].lock().unwrap());

    println!("Hello, world!");
}
