use super::prelude::*;

#[derive(Debug)]
pub struct ServiceProvider {
    pub(crate) base: BaseInstance,
}

impl ServiceProvider {
    pub(crate) fn new(name: &'static str, class: ClassName) -> Self {
        Self {
            base: BaseInstance::new(name, class),
        }
    }
}

impl ServiceProvider {
    pub fn find_service(&self, class: ClassName) -> Result<Instance, GetServiceFrror> {
        if !class.is_service() {
            return Err(GetServiceFrror::NotService);
        }
        if let Some(service) = self.find_first_child_of_class(class) {
            Ok(service)
        } else {
            Err(GetServiceFrror::NotService)
        }
    }

    pub fn get_service(&self, class: ClassName) -> Result<Instance, GetServiceFrror> {
        if !class.is_service() {
            return Err(GetServiceFrror::NotService);
        }
        if let Some(service) = self.find_first_child_of_class(class) {
            Ok(service)
        } else {
            Ok(match class {
                ClassName::Workspace => Instance::new::<Workspace>(Some(self.get_self_ptr())),
                ClassName::RibbonManager => {
                    Instance::new::<RibbonManager>(Some(self.get_self_ptr()))
                }
                _ => unimplemented!("unimplemented class to create {}", class),
            })
        }
    }
}

impl DefaultClassName for ServiceProvider {
    fn default_class_name() -> ClassName {
        ClassName::ServiceProvider
    }
}

impl AnyInstance for ServiceProvider {
    fn base(&self) -> &BaseInstance {
        self.base.base()
    }

    fn base_mut(&mut self) -> &mut BaseInstance {
        self.base.base_mut()
    }
}

ribbon_oop::impl_castable!(ServiceProvider, {
    BaseInstance,
});
