use bevy::prelude::*;

use crate::plugins::{
    game::building_placement::PlacementState, ui::sidebar_setup::enums::icon::SidebarBuildButton,
};

pub fn button_click_system(
    mut placement_state: ResMut<PlacementState>,
    query: Query<(&Interaction, &SidebarBuildButton), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, button) in &query {
        if *interaction == Interaction::Pressed {
            match button {
                SidebarBuildButton::Gwepicon => {
                    placement_state.active = true;
                }
            }
        }
    }
}
