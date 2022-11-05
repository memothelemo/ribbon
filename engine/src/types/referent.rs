use std::num::NonZeroU128;

/// An unique identifier for Roblox instances.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Referent(u128);

impl std::fmt::Debug for Referent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<32x}", self.0)
    }
}

impl Default for Referent {
    fn default() -> Self {
        Self(rand::random::<NonZeroU128>().get())
    }
}

impl Referent {
    /// Generates and creates a new unique identifier
    /// for any Roblox instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an [object](Referent) predefined
    /// by a [u128] number in a parameter.
    ///
    /// It will return [`None`] if the value is 0.
    pub const fn new_from_u128(number: u128) -> Option<Self> {
        if number == 0 {
            None
        } else {
            Some(Self(number))
        }
    }

    /// ## Safety
    ///
    /// The value must not be 0.
    pub const unsafe fn new_unchecked(number: u128) -> Self {
        Self(number)
    }
}

impl Referent {
    pub fn get(&self) -> u128 {
        self.0
    }
}
