use super::ServiceProvider;
use crate::instance::Instance;

#[derive(Debug)]
pub struct DataModel<'lua> {
    pub provider: ServiceProvider,
    pub creator_id: Option<usize>,
    pub close_binders: Vec<mlua::Function<'lua>>,
    pub game_id: Option<usize>,
    pub is_sf_flags_loaded: bool,
    pub job_id: Option<String>,
    pub place_id: Option<usize>,
    pub place_version: Option<usize>,
    pub private_server_id: Option<String>,
    pub private_server_owner_id: Option<usize>,
    // SAFETY: it is only tied to Workspace object
    pub workspace: Instance,
}

impl<'lua> DataModel<'lua> {
    pub fn bind_to_close(&mut self, callback: mlua::Function<'lua>) {
        self.close_binders.push(callback);
    }

    pub fn is_loaded(&self) -> bool {
        true
    }

    pub fn set_place_id(&mut self, place_id: usize) {
        self.place_id = Some(place_id);
    }

    pub fn set_universe_id(&mut self, universe_id: usize) {
        self.game_id = Some(universe_id);
    }
}
