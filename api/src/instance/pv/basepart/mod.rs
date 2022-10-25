use crate::instance::prelude::*;

mod part;
pub use part::*;

pub struct BasePartImpl {
    pub(crate) pv: PVInstanceImpl,
    pub(crate) anchored: bool,
    pub(crate) assembly_angular_velocity: Vector3,
    pub(crate) assembly_linear_velocity: Vector3,
    pub(crate) assembly_mass: Option<f64>,
    // SAFETY: It is restricted to BasePart only...
    pub(crate) assembly_root_part: Option<InstanceRef>,
    // TODO: BackSurface
    // TODO: BottomSurface
    pub(crate) brick_color: BrickColor,
    pub(crate) cframe: CFrame,
    pub(crate) can_collide: bool,
    pub(crate) can_query: bool,
    pub(crate) can_touch: bool,
    pub(crate) cast_shadow: bool,
    pub(crate) center_of_mass: Vector3,
    pub(crate) collision_group_id: Option<u64>,
    pub(crate) color: Color3,
    pub(crate) current_physical_properties: PhysicalProperties,
    pub(crate) custom_physical_properties: PhysicalProperties,
    // TODO: FrontSurface
    // TODO: LeftSurface
    pub(crate) locked: bool,
    pub(crate) mass: f64,
    pub(crate) massless: bool,
    // TODO: Material
    pub(crate) material_variant: String,
    pub(crate) orientation: Vector3,
    pub(crate) pivot_offset: CFrame,
    pub(crate) position: Vector3,
    pub(crate) reflectance: f64,
    pub(crate) resize_increment: f64,
    pub(crate) resizeable_faces: Faces,
    // TODO: RightSurface
    pub(crate) root_priority: u64,
    pub(crate) rotation: Vector3,
    pub(crate) size: Vector3,
    // TODO: TopSurface
    pub(crate) transparency: f64,
}

impl BasePartImpl {
    pub(crate) async fn new(name: &'static str, parent: Option<InstanceRef>, arena: InstanceArena) -> Self {
        Self {
            pv: PVInstanceImpl::new(name, parent, arena),
            anchored: false,
            assembly_angular_velocity: self.assembly_angular_velocity,
            assembly_linear_velocity: self.assembly_linear_velocity,
            assembly_mass: self.assembly_mass,
            assembly_root_part: self.assembly_root_part.clone(),
            brick_color: self.brick_color,
            cframe: self.cframe,
            can_collide: self.can_collide,
            can_query: self.can_query,
            can_touch: self.can_touch,
            cast_shadow: self.cast_shadow,
            center_of_mass: self.center_of_mass,
            collision_group_id: self.collision_group_id,
            color: self.color,
            current_physical_properties: self.current_physical_properties,
            custom_physical_properties: self.custom_physical_properties,
            locked: self.locked,
            mass: self.mass,
            massless: self.massless,
            material_variant: self.material_variant.clone(),
            orientation: self.orientation,
            pivot_offset: self.pivot_offset,
            position: self.position,
            reflectance: self.reflectance,
            resize_increment: self.resize_increment,
            resizeable_faces: self.resizeable_faces,
            root_priority: self.root_priority,
            rotation: self.rotation,
            size: self.size,
            transparency: self.transparency,
        }
    }

    pub(crate) async fn clone(&self, arena: InstanceArena) -> Result<Self, InstanceCloneError> {
        let pv = PVInstanceImpl::clone(&self.pv, arena).await?;
        Ok(Self {
            pv,
            anchored: self.anchored,
            assembly_angular_velocity: self.assembly_angular_velocity,
            assembly_linear_velocity: self.assembly_linear_velocity,
            assembly_mass: self.assembly_mass,
            assembly_root_part: self.assembly_root_part.clone(),
            brick_color: self.brick_color,
            cframe: self.cframe,
            can_collide: self.can_collide,
            can_query: self.can_query,
            can_touch: self.can_touch,
            cast_shadow: self.cast_shadow,
            center_of_mass: self.center_of_mass,
            collision_group_id: self.collision_group_id,
            color: self.color,
            current_physical_properties: self.current_physical_properties,
            custom_physical_properties: self.custom_physical_properties,
            locked: self.locked,
            mass: self.mass,
            massless: self.massless,
            material_variant: self.material_variant.clone(),
            orientation: self.orientation,
            pivot_offset: self.pivot_offset,
            position: self.position,
            reflectance: self.reflectance,
            resize_increment: self.resize_increment,
            resizeable_faces: self.resizeable_faces,
            root_priority: self.root_priority,
            rotation: self.rotation,
            size: self.size,
            transparency: self.transparency,
        })
    }
}

pub trait BasePart: PVInstance {
    fn _base_part(&self) -> &BasePartImpl;
    fn _base_part_mut(&mut self) -> &mut BasePartImpl;
}
