use std::path::Path;

use bevy::prelude::*;

use crate::plugins::shp::shp_reader;

pub fn setup_sidebar(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // Fullscreen UI root
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            // Right sidebar
            parent
                .spawn((
                    Node {
                        width: Val::Px(168.0),
                        height: Val::Percent(100.0),

                        position_type: PositionType::Absolute,
                        right: Val::Px(0.0),

                        flex_direction: FlexDirection::Column,

                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                ))
                .with_children(|sidebar| {
                    //Sidebar image
                    //Tabs.
                    if let Ok(handles) = shp_reader::decode_shp_to_image(
                        &mut images,
                        Path::new("assets/shp/ui/sidebar/credits.shp"),
                        Path::new("assets/shp/ui/sidebar/sidebar.pal"),
                        false,
                    ) {
                        if let Some(handle) = handles.get(0) {
                            sidebar.spawn((
                                ImageNode::new(handle.clone()),
                                Node {
                                    width: Val::Px(168.0),
                                    height: Val::Px(16.0),
                                    ..default()
                                },
                            ));
                        }
                    }
                    //Top.
                    if let Ok(handles) = shp_reader::decode_shp_to_image(
                        &mut images,
                        Path::new("assets/shp/ui/sidebar/top.shp"),
                        Path::new("assets/shp/ui/sidebar/sidebar.pal"),
                        false,
                    ) {
                        if let Some(handle) = handles.get(0) {
                            sidebar
                                .spawn((
                                    ImageNode::new(handle.clone()),
                                    Node {
                                        width: Val::Px(168.0),
                                        height: Val::Px(32.0),
                                        ..default()
                                    },
                                ))
                                .with_children(|top| {
                                    top.spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(72.0),
                                            height: Val::Px(22.0),
                                            top: Val::Px(5.0),
                                            left: Val::Px(13.0),
                                            ..default()
                                        },
                                    ))
                                    .with_children(|button| {
                                        if let Ok(handles) = shp_reader::decode_shp_to_image(
                                            &mut images,
                                            Path::new("assets/shp/ui/sidebar/diplobtn.shp"),
                                            Path::new("assets/shp/ui/sidebar/sidebar.pal"),
                                            false,
                                        ) {
                                            if let Some(handle) = handles.get(0) {
                                                button.spawn((
                                                    ImageNode::new(handle.clone()),
                                                    Node {
                                                        width: Val::Percent(100.0),
                                                        height: Val::Percent(100.0),
                                                        ..default()
                                                    },
                                                ));
                                            }
                                        }
                                    });
                                    top.spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(72.0),
                                            height: Val::Px(22.0),
                                            top: Val::Px(5.0),
                                            left: Val::Px(13.0),
                                            ..default()
                                        },
                                    ))
                                    .with_children(|button| {
                                        if let Ok(handles) = shp_reader::decode_shp_to_image(
                                            &mut images,
                                            Path::new("assets/shp/ui/sidebar/optbtn.shp"),
                                            Path::new("assets/shp/ui/sidebar/sidebar.pal"),
                                            false,
                                        ) {
                                            if let Some(handle) = handles.get(0) {
                                                button.spawn((
                                                    ImageNode::new(handle.clone()),
                                                    Node {
                                                        width: Val::Percent(100.0),
                                                        height: Val::Percent(100.0),
                                                        ..default()
                                                    },
                                                ));
                                            }
                                        }
                                    });
                                });
                        }
                    }
                    //Radar.
                    if let Ok(handles) = shp_reader::decode_shp_to_image(
                        &mut images,
                        Path::new("assets/shp/ui/sidebar/radar.shp"),
                        Path::new("assets/shp/ui/sidebar/sidebar.pal"),
                        false,
                    ) {
                        if let Some(handle) = handles.get(0) {
                            sidebar.spawn((
                                ImageNode::new(handle.clone()),
                                Node {
                                    width: Val::Px(168.0),
                                    height: Val::Px(110.0),
                                    ..default()
                                },
                            ));
                        }
                    }
                    //Side1.
                    if let Ok(handles) = shp_reader::decode_shp_to_image(
                        &mut images,
                        Path::new("assets/shp/ui/sidebar/side1.shp"),
                        Path::new("assets/shp/ui/sidebar/sidebar.pal"),
                        false,
                    ) {
                        if let Some(handle) = handles.get(0) {
                            sidebar
                                .spawn((
                                    ImageNode::new(handle.clone()),
                                    Node {
                                        width: Val::Px(168.0),
                                        height: Val::Px(69.0),
                                        ..default()
                                    },
                                ))
                                .with_children(|side1| {
                                    //Repair.
                                    side1
                                        .spawn((
                                            Button,
                                            Node {
                                                width: Val::Px(52.0),
                                                height: Val::Px(32.0),
                                                top: Val::Px(8.0),
                                                left: Val::Px(33.0),
                                                ..default()
                                            },
                                        ))
                                        .with_children(|button| {
                                            if let Ok(handles) = shp_reader::decode_shp_to_image(
                                                &mut images,
                                                Path::new("assets/shp/ui/sidebar/repair.shp"),
                                                Path::new("assets/shp/ui/sidebar/sidebar.pal"),
                                                false,
                                            ) {
                                                if let Some(handle) = handles.get(0) {
                                                    button.spawn((
                                                        ImageNode::new(handle.clone()),
                                                        Node {
                                                            width: Val::Percent(100.0),
                                                            height: Val::Percent(100.0),
                                                            ..default()
                                                        },
                                                    ));
                                                }
                                            }
                                        });
                                    //Sell.
                                    side1
                                        .spawn((
                                            Button,
                                            Node {
                                                width: Val::Px(52.0),
                                                height: Val::Px(32.0),
                                                top: Val::Px(8.0),
                                                left: Val::Px(33.0),
                                                ..default()
                                            },
                                        ))
                                        .with_children(|button| {
                                            if let Ok(handles) = shp_reader::decode_shp_to_image(
                                                &mut images,
                                                Path::new("assets/shp/ui/sidebar/sell.shp"),
                                                Path::new("assets/shp/ui/sidebar/sidebar.pal"),
                                                false,
                                            ) {
                                                if let Some(handle) = handles.get(0) {
                                                    button.spawn((
                                                        ImageNode::new(handle.clone()),
                                                        Node {
                                                            width: Val::Percent(100.0),
                                                            height: Val::Percent(100.0),
                                                            ..default()
                                                        },
                                                    ));
                                                }
                                            }
                                        });
                                    //Tab00.
                                    side1
                                        .spawn((
                                            Button,
                                            Node {
                                                width: Val::Px(32.0),
                                                height: Val::Px(28.0),
                                                top: Val::Px(39.0),
                                                left: Val::Px(20.0),
                                                position_type: PositionType::Absolute,
                                                ..default()
                                            },
                                        ))
                                        .with_children(|button| {
                                            if let Ok(handles) = shp_reader::decode_shp_to_image(
                                                &mut images,
                                                Path::new("assets/shp/ui/sidebar/tab00.shp"),
                                                Path::new("assets/shp/ui/sidebar/sidebar.pal"),
                                                false,
                                            ) {
                                                if let Some(handle) = handles.get(0) {
                                                    button.spawn((
                                                        ImageNode::new(handle.clone()),
                                                        Node {
                                                            width: Val::Percent(100.0),
                                                            height: Val::Percent(100.0),
                                                            ..default()
                                                        },
                                                    ));
                                                }
                                            }
                                        });
                                    //Tab01.
                                    side1
                                        .spawn((
                                            Button,
                                            Node {
                                                width: Val::Px(32.0),
                                                height: Val::Px(28.0),
                                                top: Val::Px(39.0),
                                                left: Val::Px(52.0),
                                                position_type: PositionType::Absolute,
                                                ..default()
                                            },
                                        ))
                                        .with_children(|button| {
                                            if let Ok(handles) = shp_reader::decode_shp_to_image(
                                                &mut images,
                                                Path::new("assets/shp/ui/sidebar/tab01.shp"),
                                                Path::new("assets/shp/ui/sidebar/sidebar.pal"),
                                                false,
                                            ) {
                                                if let Some(handle) = handles.get(0) {
                                                    button.spawn((
                                                        ImageNode::new(handle.clone()),
                                                        Node {
                                                            width: Val::Percent(100.0),
                                                            height: Val::Percent(100.0),
                                                            ..default()
                                                        },
                                                    ));
                                                }
                                            }
                                        });
                                    //Tab02.
                                    side1
                                        .spawn((
                                            Button,
                                            Node {
                                                width: Val::Px(32.0),
                                                height: Val::Px(28.0),
                                                top: Val::Px(39.0),
                                                left: Val::Px(84.0),
                                                position_type: PositionType::Absolute,
                                                ..default()
                                            },
                                        ))
                                        .with_children(|button| {
                                            if let Ok(handles) = shp_reader::decode_shp_to_image(
                                                &mut images,
                                                Path::new("assets/shp/ui/sidebar/tab02.shp"),
                                                Path::new("assets/shp/ui/sidebar/sidebar.pal"),
                                                false,
                                            ) {
                                                if let Some(handle) = handles.get(0) {
                                                    button.spawn((
                                                        ImageNode::new(handle.clone()),
                                                        Node {
                                                            width: Val::Percent(100.0),
                                                            height: Val::Percent(100.0),
                                                            ..default()
                                                        },
                                                    ));
                                                }
                                            }
                                        });
                                    //Tab03.
                                    side1
                                        .spawn((
                                            Button,
                                            Node {
                                                width: Val::Px(32.0),
                                                height: Val::Px(28.0),
                                                top: Val::Px(39.0),
                                                left: Val::Px(116.0),
                                                position_type: PositionType::Absolute,
                                                ..default()
                                            },
                                        ))
                                        .with_children(|button| {
                                            if let Ok(handles) = shp_reader::decode_shp_to_image(
                                                &mut images,
                                                Path::new("assets/shp/ui/sidebar/tab03.shp"),
                                                Path::new("assets/shp/ui/sidebar/sidebar.pal"),
                                                false,
                                            ) {
                                                if let Some(handle) = handles.get(0) {
                                                    button.spawn((
                                                        ImageNode::new(handle.clone()),
                                                        Node {
                                                            width: Val::Percent(100.0),
                                                            height: Val::Percent(100.0),
                                                            ..default()
                                                        },
                                                    ));
                                                }
                                            }
                                        });
                                });
                        }
                    }
                    let mut i = 0;
                    while i < 9 {
                        //Side2.
                        if let Ok(handles) = shp_reader::decode_shp_to_image(
                            &mut images,
                            Path::new("assets/shp/ui/sidebar/side2.shp"),
                            Path::new("assets/shp/ui/sidebar/sidebar.pal"),
                            false,
                        ) {
                            if let Some(handle) = handles.get(0) {
                                sidebar.spawn((
                                    ImageNode::new(handle.clone()),
                                    Node {
                                        width: Val::Px(168.0),
                                        height: Val::Px(50.0),
                                        ..default()
                                    },
                                ));
                            }
                        }
                        i += 1;
                    }
                    //Side3.
                    if let Ok(handles) = shp_reader::decode_shp_to_image(
                        &mut images,
                        Path::new("assets/shp/ui/sidebar/side3.shp"),
                        Path::new("assets/shp/ui/sidebar/sidebar.pal"),
                        false,
                    ) {
                        if let Some(handle) = handles.get(0) {
                            sidebar.spawn((
                                ImageNode::new(handle.clone()),
                                Node {
                                    width: Val::Px(168.0),
                                    height: Val::Px(26.0),
                                    ..default()
                                },
                            ));
                        }
                    }
                    //addon.
                    if let Ok(handles) = shp_reader::decode_shp_to_image(
                        &mut images,
                        Path::new("assets/shp/ui/sidebar/addon.shp"),
                        Path::new("assets/shp/ui/sidebar/sidebar.pal"),
                        false,
                    ) {
                        if let Some(handle) = handles.get(0) {
                            sidebar.spawn((
                                ImageNode::new(handle.clone()),
                                Node {
                                    width: Val::Px(168.0),
                                    height: Val::Px(63.0),
                                    ..default()
                                },
                            ));
                        }
                    }
                });
        });
}
