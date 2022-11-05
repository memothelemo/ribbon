#[derive(Debug, PartialEq, Eq)]
pub enum LuaClassName {
    BasePart,
    BaseScript,
    DataModel,
    HttpService,
    LuaSourceContainer,
    Part,
    PVInstance,
    Script,
    ServiceProvider,
}

impl LuaClassName {
    pub fn can_be_downcasted(&self, class: LuaClassName) -> bool {
        if self.eq(&class) {
            return true;
        }
        matches!(
            (self, class),
            (Self::BasePart, Self::PVInstance)
                | (Self::Part, Self::BasePart | Self::PVInstance)
                | (Self::BaseScript, Self::LuaSourceContainer)
                | (Self::Script, Self::BaseScript | Self::LuaSourceContainer)
                | (Self::DataModel, Self::ServiceProvider)
        )
    }
}
