use super::prelude::*;

#[derive(Debug)]
pub struct Part {
    pub(crate) base: BasePart,
}

impl Part {
    pub(crate) fn new(name: &'static str, class: ClassName) -> Self {
        Self {
            base: BasePart::new(name, class),
        }
    }
}

impl CreatableInstance for Part {
    fn create(_parent: Option<Instance>) -> Instance {
        Instance::new_from_trait(Self::new("Part", ClassName::Part))
    }
}

impl DefaultClassName for Part {
    fn default_class_name() -> ClassName {
        ClassName::Part
    }
}

impl AnyInstance for Part {
    fn base(&self) -> &super::BaseInstance {
        self.base.base()
    }

    fn base_mut(&mut self) -> &mut super::BaseInstance {
        self.base.base_mut()
    }
}

impl Sealed for Part {}
default_instance_lua_impl!(Part);

ribbon_oop::impl_castable!(Part, {
    BasePart,
    PVInstance,
    BaseInstance,
});
