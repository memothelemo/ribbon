/// It represents a vector in 3D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vector3 {
    fn default() -> Self {
        Self::new(0., 0., 0.)
    }
}

impl Vector3 {
    /// Creates a new [Vector3] object.
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn x(&self) -> f64 {
        self.x
    }

    pub const fn y(&self) -> f64 {
        self.y
    }

    pub const fn z(&self) -> f64 {
        self.z
    }
}
