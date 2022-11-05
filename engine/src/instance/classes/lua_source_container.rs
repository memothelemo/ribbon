use super::BaseInstance;

#[derive(Debug)]
pub struct LuaSourceContainer {
    pub base: BaseInstance,
}

impl LuaSourceContainer {
    pub fn new(name: &'static str) -> Self {
        LuaSourceContainer {
            base: BaseInstance::new(name),
        }
    }
}
