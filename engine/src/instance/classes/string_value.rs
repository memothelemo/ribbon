use super::prelude::*;

#[derive(Debug)]
pub struct StringValue {
    pub(crate) base: ValueBase,
    value: String,
}

impl StringValue {
    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
    }
}

impl CreatableInstance for StringValue {
    fn create(_parent: Option<Instance>) -> Instance {
        Instance::new_from_trait(Self {
            base: ValueBase::new("StringValue", ClassName::StringValue),
            value: String::new(),
        })
    }
}

impl DefaultClassName for StringValue {
    fn default_class_name() -> ClassName {
        ClassName::StringValue
    }
}

impl AnyInstance for StringValue {
    fn base(&self) -> &super::BaseInstance {
        self.base.base()
    }

    fn base_mut(&mut self) -> &mut super::BaseInstance {
        self.base.base_mut()
    }
}

ribbon_oop::impl_castable!(StringValue, {
    ValueBase,
    BaseInstance,
});
