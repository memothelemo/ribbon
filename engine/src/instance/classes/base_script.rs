use super::LuaSourceContainer;

#[derive(Debug)]
pub struct BaseScript {
    pub lsc: LuaSourceContainer,
    pub enabled: bool,
}

impl BaseScript {
    pub fn new(name: &'static str) -> Self {
        BaseScript {
            lsc: LuaSourceContainer::new(name),
            enabled: true,
        }
    }
}
