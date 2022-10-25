use crate::instance::prelude::*;
use crate::InstanceRef;

pub struct BaseInstance {
    // practically useless but useful later on
    pub(crate) id: Ref,
    pub(crate) arena_id: OnceCell<InstanceRef>,

    pub(crate) name: String,
    pub(crate) children: Vec<InstanceRef>,
    pub(crate) parent: Option<InstanceRef>,
}

impl Default for BaseInstance {
    fn default() -> Self {
        Self {
            id: Ref::new(),
            arena_id: OnceCell::new(),

            name: String::from("Instance"),
            children: Vec::new(),
            parent: None,
        }
    }
}

impl BaseInstance {
    pub(crate) fn new(name: &'static str, parent: Option<InstanceRef>) -> Self {
        Self{
            name: name.to_string(),
            parent,
            ..Default::default()
        }
    }

    pub(crate) async fn clone(&self, arena: InstanceArena) -> Result<Self, InstanceCloneError> {
        // generate new children
        let mut children = Vec::new();
        for child in self.children.iter() {
            let arena_ref = arena.read().await;
            let child = arena_ref[*child].lock().await;
            println!("RACE CONDITION #2");
            children.push(child.clone(arena.clone()).await?);
            println!("RACE CONDITION #2 done");
        }
        
        // no need to duplicate its parent...
        Ok(Self {
            id: Ref::new(),
            arena_id: OnceCell::new(),
            name: self.name.to_string(),
            children,
            parent: None,
        })
    }
}
