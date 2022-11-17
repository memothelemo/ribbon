pub trait DefaultClassName: Sized + 'static {
    fn default_class_name() -> ClassName;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
pub enum ClassName {
    BaseInstance,

      PVInstance,
        BasePart,
          Part,
            FormFactorPart,

      Model,
        WorldRoot,
          Workspace,

      ServiceProvider,
        DataModel,

      ValueBase,
        StringValue,

      RibbonManager,

      Cloud,
}

impl ClassName {
    pub fn is_service(&self) -> bool {
        matches!(self, Self::RibbonManager | Self::Workspace)
    }
}

impl std::fmt::Display for ClassName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BaseInstance => write!(f, "BaseInstance"),

            Self::PVInstance => write!(f, "PVInstance"),
            Self::BasePart => write!(f, "BasePart"),
            Self::Part => write!(f, "Part"),
            Self::FormFactorPart => write!(f, "FormFactorPart"),

            Self::ServiceProvider => write!(f, "ServiceProvider"),
            Self::DataModel => write!(f, "DataModel"),

            Self::ValueBase => write!(f, "ValueBase"),
            Self::StringValue => write!(f, "StringValue"),

            Self::RibbonManager => write!(f, "RibbonManager"),

            Self::Model => write!(f, "Model"),
            Self::WorldRoot => write!(f, "WorldRoot"),
            Self::Workspace => write!(f, "Workspace"),

            Self::Cloud => write!(f, "Clouds"),
        }
    }
}

impl ClassName {
    // Upcasting is not supported due to complications with Rust but maybe soon! :)
    // /// ```txt
    // ///          origin (&self)
    // /// Instance   ⬇             target
    // ///     <- PVInstance         ⬇
    // ///         <- (BasePart <- Part <- FormFactorPart)
    // ///         └- (Model <- Workspace);
    // ///     <- Clouds
    // /// ```
    // pub fn can_upcast(&self, target: ClassName) -> bool {
    //     if target == *self {
    //         return true;
    //     }
    //     #[allow(clippy::match_like_matches_macro)]
    //     match (target, self) {
    //         (Self::FormFactorPart, Self::Part | Self::BasePart | Self::PVInstance) => true,
    //         (Self::Part, Self::BasePart | Self::PVInstance) => true,

    //         (Self::Workspace, Self::Model | Self::PVInstance) => true,
    //         (Self::Model, Self::PVInstance) => true,

    //         _ => false,
    //     }
    // }

    /// ```txt
    ///          target
    /// Instance   ⬇            origin (&self)
    ///     <- PVInstance         ⬇
    ///         <- (BasePart <- Part <- FormFactorPart)
    ///         └- (Model <- WorldRoot <- Workspace);
    ///     <- ValueBase
    ///         └- StringValue
    ///     <- Clouds
    /// ```
    pub fn can_downcast(&self, target: ClassName) -> bool {
        if target == ClassName::BaseInstance {
            return true;
        }
        if target == *self {
            return true;
        }
        #[allow(clippy::match_like_matches_macro)]
        match (self, target) {
            (Self::FormFactorPart, Self::Part | Self::BasePart | Self::PVInstance) => true,
            (Self::Part, Self::BasePart | Self::PVInstance) => true,

            (Self::Workspace, Self::WorldRoot | Self::Model | Self::PVInstance) => true,
            (Self::Model, Self::PVInstance) => true,

            (Self::StringValue, Self::ValueBase) => true,

            (Self::DataModel, Self::ServiceProvider) => true,

            _ => false,
        }
    }
}
