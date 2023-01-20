use strum::{Display, EnumString};

pub trait DefaultClassName: Sized + 'static {
    fn default_class_name() -> ClassName;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display)]
#[rustfmt::skip]
pub enum ClassName {
    BaseInstance,

      PVInstance,
        BasePart,
          Part, // creatable
            FormFactorPart,

      Model, // creatable
        WorldRoot,
          Workspace,

      ServiceProvider,
        DataModel,

      ValueBase,
        StringValue, // creatable

      RibbonManager,

      Cloud, // creatable
}

impl ClassName {
    pub fn is_service(&self) -> bool {
        matches!(self, Self::RibbonManager | Self::Workspace)
    }
}

impl ClassName {
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
