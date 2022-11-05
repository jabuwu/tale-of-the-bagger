use bevy::prelude::*;

use crate::AssetLibrary;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum VersionSystem {
    Spawn,
}

pub struct VersionPlugin;

impl Plugin for VersionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VersionSpawnEvent>()
            .add_system(version_spawn.label(VersionSystem::Spawn));
    }
}

#[derive(Default)]
pub struct VersionSpawnEvent;

#[derive(Component)]
pub struct Version;

fn version_spawn(
    mut spawn_events: EventReader<VersionSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in spawn_events.iter() {
        commands
            .spawn_bundle(VisibilityBundle::default())
            .insert_bundle(TransformBundle {
                local: Transform::from_xyz(950., -530., 0.99),
                ..Default::default()
            })
            .insert(Version)
            .with_children(|parent| {
                parent.spawn_bundle(Text2dBundle {
                    transform: Transform::from_xyz(0., 0., 0.001),
                    text: Text::from_section(
                        env!("CARGO_PKG_VERSION"),
                        TextStyle {
                            font: asset_library.fonts.default.clone(),
                            font_size: 30.,
                            color: Color::WHITE,
                        },
                    )
                    .with_alignment(TextAlignment {
                        horizontal: HorizontalAlign::Right,
                        vertical: VerticalAlign::Bottom,
                    }),
                    ..Default::default()
                });
                parent.spawn_bundle(Text2dBundle {
                    transform: Transform::from_xyz(2., -2., 0.),
                    text: Text::from_section(
                        env!("CARGO_PKG_VERSION"),
                        TextStyle {
                            font: asset_library.fonts.default.clone(),
                            font_size: 30.,
                            color: Color::BLACK,
                        },
                    )
                    .with_alignment(TextAlignment {
                        horizontal: HorizontalAlign::Right,
                        vertical: VerticalAlign::Bottom,
                    }),
                    ..Default::default()
                });
            });
    }
}
