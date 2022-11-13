use std::num::NonZeroUsize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ref(NonZeroUsize);

impl Default for Ref {
    fn default() -> Self {
        Self(rand::random())
    }
}

impl Ref {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub const fn new_with_value(n: usize) -> Option<Self> {
        if n == 0 {
            None
        } else {
            unsafe { Some(Self::new_unchecked(n)) }
        }
    }

    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub const unsafe fn new_unchecked(n: usize) -> Self {
        Self(NonZeroUsize::new_unchecked(n))
    }
}
