use crate::instance::prelude::*;

mod basepart;
pub use basepart::*;

#[derive(Default)]
pub struct PVInstance {
    pub(crate) base: BaseInstance,
    pub(crate) origin_orientation: Vector3,
    pub(crate) origin_position: Vector3,
    pub(crate) pivot_origin_orientation: Vector3,
    pub(crate) pivot_origin_position: Vector3,
}

impl PVInstance {
    pub(crate) fn new(name: &'static str, parent: Option<InstanceRef>) -> Self {
        Self {
            base: BaseInstance::new(name, parent),
            ..Default::default()
        }
    }

    pub(crate) async fn clone(&self, arena: InstanceArena) -> Result<Self, InstanceCloneError> {
        let base = BaseInstance::clone(&self.base, arena).await?;
        Ok(Self {
            base,
            origin_orientation: self.origin_orientation,
            origin_position: self.origin_position,
            pivot_origin_orientation: self.pivot_origin_orientation,
            pivot_origin_position: self.pivot_origin_position,
        })
    }
}

pub trait PVInstanceType: InstanceType {
    fn _pv(&self) -> &PVInstance;
    fn _pv_mut(&mut self) -> &mut PVInstance;

    fn origin_orientation(&self) -> Vector3 {
        self._pv().origin_orientation
    }

    fn set_origin_orientation(&mut self, vector: Vector3) {
        self._pv_mut().origin_orientation = vector;
    }

    fn origin_position(&self) -> Vector3 {
        self._pv().origin_position
    }

    fn set_origin_position(&mut self, vector: Vector3) {
        self._pv_mut().origin_position = vector;
    }

    fn pivot_origin_orientation(&self) -> Vector3 {
        self._pv().pivot_origin_orientation
    }

    fn set_pivot_origin_orientation(&mut self, vector: Vector3) {
        self._pv_mut().pivot_origin_orientation = vector;
    }

    fn pivot_origin_position(&self) -> Vector3 {
        self._pv().pivot_origin_position
    }

    fn set_pivot_origin_position(&mut self, vector: Vector3) {
        self._pv_mut().pivot_origin_position = vector;
    }
}
