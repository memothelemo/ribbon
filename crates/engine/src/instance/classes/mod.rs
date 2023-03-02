macro_rules! imports {
    ($($module:ident,)*) => {$(
        mod $module;
        pub use $module::*;
    )*};
}

imports! {
    base_instance,
    cloud,
    model,
    pv_instance,
}

#[allow(unused_imports)]
pub(crate) mod private {
    pub(crate) use super::base_instance::{set_inner_ptr, wipe_inner_ptr};
}

mod prelude {
    pub use super::BaseInstance;
    pub use crate::instance::class_name::ClassName;
    pub use crate::instance::error::InstanceError;
    pub use crate::instance::traits::*;
    pub use crate::instance::Instance;

    pub(crate) use crate::private::Sealed;

    macro_rules! impl_rest {
        ($name:ident, { parent = $parent:ident, base = $( $base:tt )* }) => {
            impl Sealed for $name {}
            impl DefaultClassName for $name {
                fn default_class_name() -> ClassName {
                    ClassName::$name
                }
            }

            impl AnyInstance for $name {
                fn base(&self) -> &BaseInstance {
                    &self.$( $base )*
                }

                fn base_mut(&mut self) -> &mut BaseInstance {
                    &mut self.$( $base )*
                }

                fn parent(&self) -> Option<&dyn AnyInstance> {
                    Some(&self.$parent)
                }

                fn parent_mut(&mut self) -> Option<&mut dyn AnyInstance> {
                    Some(&mut self.$parent)
                }
            }
        };
    }
    pub(crate) use impl_rest;
}
