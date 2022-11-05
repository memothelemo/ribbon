mod base_instance;
mod base_part;
mod base_script;
mod data_model;
mod http_service;
mod lua_source_container;
mod part;
mod pv_instance;
mod script;
mod service_provider;

pub use base_instance::*;
pub use base_part::*;
pub use base_script::*;
pub use data_model::*;
pub use http_service::*;
pub use lua_source_container::*;
pub use part::*;
pub use pv_instance::*;
pub use script::*;
pub use service_provider::*;

macro_rules! make_is_fn {
    ($function_name:ident, { $( $name:literal, )* }) => {
        pub fn $function_name(name: &str) -> bool {
            use once_cell::sync::Lazy;
            use std::collections::HashSet;

            static MAP: Lazy<HashSet<&'static str>> = Lazy::new(|| {
                #[allow(unused_mut)]
                let mut set = HashSet::new();
                $( set.insert($name); )*
                set
            });

            MAP.contains(name)
        }
    };
}

make_is_fn!(is_rbx_class_name, {
    "Instance",
        "PVInstance",
            "BasePart",
                "Part",
        "LuaSourceContainer",
            "BaseScript",
                "Script",
});

make_is_fn!(is_rbx_service, {});
