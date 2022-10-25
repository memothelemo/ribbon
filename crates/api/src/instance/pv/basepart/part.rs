use crate::instance::prelude::*;

// FormFactorPart is excluded because it's way too long
// and it is not being used most days (except legacy games/experiences)
#[derive(Default)]
pub struct Part {
    pub(crate) base_part: BasePart,
    // TODO: shape,
}

impl CreatableInstance for Part {
    fn create_impl(parent: Option<InstanceRef>) -> Self {
        Self {
            base_part: BasePart::new("Part", parent),
        }
    }
}

pub trait PartType: PVInstanceType {
    fn _part(&self) -> &Part;
    fn _part_mut(&mut self) -> &mut Part;
}

#[async_trait]
impl InstanceType for Part {
    fn _base(&self) -> &BaseInstance {
        &self.base_part.pv.base
    }

    fn _base_mut(&mut self) -> &mut BaseInstance {
        &mut self.base_part.pv.base
    }

    fn class_name(&self) -> &'static str {
        "Part"
    }

    async fn clone(&self, arena: InstanceArena) -> Result<InstanceRef, InstanceCloneError> {
        let base_part = BasePart::clone(&self.base_part, arena.clone()).await?;

        let mut arena = arena.write().await;
        let instance = arena.alloc(Arc::new(Mutex::new(Self { base_part })));
        arena[instance].lock().await._base_mut().arena_id.set(instance).unwrap();

        Ok(instance)
    }
}

impl PVInstanceType for Part {
    fn _pv(&self) -> &PVInstance {
        &self.base_part.pv
    }

    fn _pv_mut(&mut self) -> &mut PVInstance {
        &mut self.base_part.pv
    }
}

impl PartType for Part {
    fn _part(&self) -> &Part {
        self
    }

    fn _part_mut(&mut self) -> &mut Part {
        self
    }
}
