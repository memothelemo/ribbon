use crate::instance::prelude::*;

pub mod part;

#[derive(Debug)]
pub struct BasePart {
    pub(crate) pv: PVInstance,
    pub(crate) anchored: bool,
    pub(crate) assembly_angular_velocity: Vector3,
    pub(crate) assembly_linear_velocity: Vector3,
    pub(crate) assembly_mass: Option<f64>,
    // SAFETY: It is restricted to BasePart only...
    pub(crate) assembly_root_part: Option<Instance>,
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

impl BasePart {
    pub(crate) fn new(name: &'static str) -> Self {
        Self {
            pv: PVInstance::new(name),
            anchored: false,
            assembly_angular_velocity: Vector3::default(),
            assembly_linear_velocity: Vector3::default(),
            assembly_mass: Default::default(),
            // SAFETY: It is restricted to BasePart only...
            assembly_root_part: Default::default(),
            // TODO: BackSurface
            // TODO: BottomSurface
            brick_color: BrickColor::MediumStoneGrey,
            cframe: CFrame::default(),
            can_collide: true,
            can_query: true,
            can_touch: true,
            cast_shadow: true,
            center_of_mass: Vector3::default(),
            collision_group_id: Default::default(),
            color: Color3::default(),
            current_physical_properties: PhysicalProperties::default(),
            custom_physical_properties: PhysicalProperties::default(),
            // TODO: FrontSurface
            // TODO: LeftSurface
            locked: false,
            mass: 2.,
            massless: false,
            // TODO: Material
            material_variant: String::from("Plastic"),
            orientation: Vector3::default(),
            pivot_offset: CFrame::default(),
            position: Vector3::default(),
            reflectance: 0.,
            resize_increment: 1.,
            resizeable_faces: Faces::default(),
            // TODO: RightSurface
            root_priority: 0,
            rotation: Vector3::default(),
            size: Vector3::default(),
            // TODO: TopSurface
            transparency: 0.,
        }
    }

    // pub(crate) fn impl_fields<'lua, T: UserData + BasePartType, F: UserDataFields<'lua, T>>(
    //     fields: &mut F,
    // ) {
    //     PVInstance::impl_fields(fields);

    //     fields.add_field_method_get("Anchored", |_lua, this| Ok(this.anchored()));
    //     fields.add_field_method_get("AssemblyMass", |_lua, this| Ok(this.assembly_mass()));

    //     fields.add_field_method_get("CanCollide", |_lua, this| Ok(this.can_collide()));
    //     fields.add_field_method_get("CanQuery", |_lua, this| Ok(this.can_query()));
    //     fields.add_field_method_get("CanTouch", |_lua, this| Ok(this.can_touch()));

    //     fields.add_field_method_get("CollisionGroupId", |_lua, this| {
    //         Ok(this.collision_group_id())
    //     });

    //     fields.add_field_method_get("Locked", |_lua, this| Ok(this.locked()));
    //     fields.add_field_method_get("Mass", |_lua, this| Ok(this.mass()));
    //     fields.add_field_method_get("Massless", |_lua, this| Ok(this.massless()));
    //     fields.add_field_method_get("MaterialVariant", |lua, this| {
    //         lua.create_string(this.material_variant())
    //     });

    //     fields.add_field_method_get("Reflectance", |_lua, this| Ok(this.reflectance()));
    //     fields.add_field_method_get("ResizeIncrement", |_lua, this| Ok(this.resize_increment()));

    //     fields.add_field_method_get("RootPriority", |_lua, this| Ok(this.root_priority()));

    //     fields.add_field_method_get("Transparency", |_lua, this| Ok(this.transparency()));
    // }
}

pub trait BasePartType: PVInstanceType + BasePartGetter {
    fn anchored(&self) -> bool {
        self.base_part().anchored
    }

    fn assembly_angular_velocity(&self) -> Vector3 {
        self.base_part().assembly_angular_velocity
    }

    fn assembly_linear_velocity(&self) -> Vector3 {
        self.base_part().assembly_linear_velocity
    }

    fn assembly_mass(&self) -> Option<f64> {
        self.base_part().assembly_mass
    }

    fn assembly_root_part(&self) -> Option<Instance> {
        self.base_part().assembly_root_part.clone()
    }

    fn brick_color(&self) -> BrickColor {
        self.base_part().brick_color
    }

    fn cframe(&self) -> CFrame {
        self.base_part().cframe
    }

    fn can_collide(&self) -> bool {
        self.base_part().can_collide
    }

    fn can_query(&self) -> bool {
        self.base_part().can_query
    }

    fn can_touch(&self) -> bool {
        self.base_part().can_touch
    }

    fn cast_shadow(&self) -> bool {
        self.base_part().cast_shadow
    }

    fn center_of_mass(&self) -> Vector3 {
        self.base_part().center_of_mass
    }

    fn collision_group_id(&self) -> Option<u64> {
        self.base_part().collision_group_id
    }

    fn color(&self) -> Color3 {
        self.base_part().color
    }

    fn current_physical_properties(&self) -> PhysicalProperties {
        self.base_part().current_physical_properties
    }

    fn custom_physical_properties(&self) -> PhysicalProperties {
        self.base_part().custom_physical_properties
    }

    fn locked(&self) -> bool {
        self.base_part().locked
    }

    fn mass(&self) -> f64 {
        self.base_part().mass
    }

    fn massless(&self) -> bool {
        self.base_part().massless
    }

    fn material_variant(&self) -> &str {
        self.base_part().material_variant.as_ref()
    }

    fn orientation(&self) -> Vector3 {
        self.base_part().orientation
    }

    fn pivot_offset(&self) -> CFrame {
        self.base_part().pivot_offset
    }

    fn position(&self) -> Vector3 {
        self.base_part().position
    }

    fn reflectance(&self) -> f64 {
        self.base_part().reflectance
    }

    fn resize_increment(&self) -> f64 {
        self.base_part().resize_increment
    }

    fn resizeable_faces(&self) -> Faces {
        self.base_part().resizeable_faces
    }

    fn root_priority(&self) -> u64 {
        self.base_part().root_priority
    }

    fn rotation(&self) -> Vector3 {
        self.base_part().rotation
    }

    fn size(&self) -> Vector3 {
        self.base_part().size
    }

    fn transparency(&self) -> f64 {
        self.base_part().transparency
    }

    // TODO: BackSurface
    // TODO: BottomSurface
    // TODO: FrontSurface
    // TODO: LeftSurface
    // TODO: Material
    // TODO: RightSurface
    // TODO: TopSurface
}
