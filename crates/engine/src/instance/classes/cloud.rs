use super::prelude::*;

#[derive(Debug)]
pub struct Clouds {
    base: BaseInstance,
    enabled: bool,
}

impl CreatableInstance for Clouds {
    fn create() -> Self {
        Self {
            base: BaseInstance::create("Clouds", ClassName::Clouds),
            enabled: true,
        }
    }
}

impl_rest!(Clouds, {
    parent = base,
    base = base
});
