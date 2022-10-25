use crate::instance::prelude::*;

mod basepart;
pub use basepart::*;

pub struct PVInstanceImpl {
    pub(crate) base: BaseInstanceImpl,
    pub(crate) origin_orientation: Vector3,
    pub(crate) origin_position: Vector3,
    pub(crate) pivot_origin_orientation: Vector3,
    pub(crate) pivot_origin_position: Vector3,
}

impl PVInstanceImpl {
    pub(crate) async fn new(name: &'static str, parent: Option<InstanceRef>, arena: InstanceArena) -> Self {
        Self {
            base: BaseInstanceImpl::new(name, parent, arena).await,
            origin_orientation: Vector3::new(0., 0., 0.),
            origin_position: Vector3::new(0., 0., 0.),
            pivot_origin_orientation: Vector3::new(0., 0., 0.),
            pivot_origin_position: Vector3::new(0., 0., 0.),
        }
    }

    pub(crate) async fn clone(&self, arena: InstanceArena) -> Result<Self, InstanceCloneError> {
        let base = BaseInstanceImpl::clone(&self.base, arena).await?;
        Ok(Self {
            base,
            origin_orientation: self.origin_orientation,
            origin_position: self.origin_position,
            pivot_origin_orientation: self.pivot_origin_orientation,
            pivot_origin_position: self.pivot_origin_position,
        })
    }
}

pub trait PVInstance: Instance {
    fn _pv(&self) -> &PVInstanceImpl;
    fn _pv_mut(&mut self) -> &mut PVInstanceImpl;

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
