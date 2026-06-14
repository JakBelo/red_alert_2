use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlacementState {
    pub active: bool,
}

#[derive(Component)]
pub struct PlacementPreview;

pub fn placement_system(
    mut commands: Commands,
    placement_state: Res<PlacementState>,
    preview_query: Query<Entity, With<PlacementPreview>>,
) {
    if !placement_state.active {
        return;
    }

    if preview_query.is_empty() {
        println!("Spawn preview");

        commands.spawn((
            Sprite {
                color: Color::srgba(0.0, 1.0, 0.0, 0.5),
                custom_size: Some(Vec2::new(64.0, 32.0)),
                ..default()
            },
            PlacementPreview,
        ));
    }
}

pub fn update_preview_position_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut preview_q: Query<&mut Transform, With<PlacementPreview>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    let grid_size = 32.0;

    let snapped_x = (world_pos.x / grid_size).floor() * grid_size;

    let snapped_y = (world_pos.y / grid_size).floor() * grid_size;

    for mut transform in &mut preview_q {
        transform.translation.x = snapped_x;
        transform.translation.y = snapped_y;
        transform.translation.z = 0.0;
    }
}
