use crate::instance::prelude::*;

// FormFactorPart is excluded because it's way too long
// and it is not being used most days (except legacy games)
pub struct PartImpl {
    pub(crate) base_part: BasePartImpl,
    // TODO: shape,
}

pub trait Part: PVInstance {
    fn _part(&self) -> &PartImpl;
    fn _part_mut(&mut self) -> &mut PartImpl;
}

#[async_trait]
impl Instance for PartImpl {
    fn _base(&self) -> &BaseInstanceImpl {
        &self.base_part.pv.base
    }

    fn _base_mut(&mut self) -> &mut BaseInstanceImpl {
        &mut self.base_part.pv.base
    }

    fn class_name(&self) -> &'static str {
        "Part"
    }

    async fn clone(&self, arena: InstanceArena) -> Result<InstanceRef, InstanceCloneError> {
        let base_part = BasePartImpl::clone(&self.base_part, arena.clone()).await?;
        let instance = arena.lock().await.alloc(Arc::new(Mutex::new(Self { base_part })));
        arena.lock().await[instance].lock().await._base_mut().arena_id.set(instance).unwrap();
        Ok(instance)
    }
}

impl PVInstance for PartImpl {
    fn _pv(&self) -> &PVInstanceImpl {
        &self.base_part.pv
    }

    fn _pv_mut(&mut self) -> &mut PVInstanceImpl {
        &mut self.base_part.pv
    }
}

impl Part for PartImpl {
    fn _part(&self) -> &PartImpl {
        self
    }

    fn _part_mut(&mut self) -> &mut PartImpl {
        self
    }
}
