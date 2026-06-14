use bevy::prelude::*;

mod sidebar_setup;
mod system;

pub struct Ui;

impl Plugin for Ui {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sidebar_setup::setup_sidebar)
            .add_systems(Update, system::button_click_system);
    }
}
