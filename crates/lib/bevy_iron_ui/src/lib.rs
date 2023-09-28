use core::{UiManager, ThemeFn};
use bevy::prelude::*;

mod systems;
pub mod layout;
pub mod core;

pub struct UiPlugin{
    pub theme: ThemeFn,
}
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(UiManager::new(self.theme))
            .add_systems(Update, systems::button_system)
            .add_systems(PostUpdate, systems::delete_marked_ui);
    }
}