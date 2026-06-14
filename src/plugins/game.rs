use bevy::prelude::*;

pub mod building_placement;
mod spawn_sprite;

pub struct Systems;

impl Plugin for Systems {
    fn build(&self, app: &mut App) {
        app.init_resource::<building_placement::PlacementState>();
        app.add_systems(Startup, spawn_sprite::setup)
            .add_systems(Update, spawn_sprite::animate_sprite)
            .add_systems(Update, building_placement::placement_system)
            .add_systems(Update, building_placement::update_preview_position_system);
    }
}
