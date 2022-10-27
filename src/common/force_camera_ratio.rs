use bevy::{prelude::*, transform::TransformSystem};

use super::transform2::Transform2System;

const DESIRED_SIZE: Vec2 = Vec2::new(1920., 1080.);
const DESIRED_RATIO: f32 = DESIRED_SIZE.x / DESIRED_SIZE.y;

const RATIO_BAR_SIZE: f32 = 10000.;

pub struct ForceCameraRatioPlugin;

impl Plugin for ForceCameraRatioPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(force_camera_ratio_setup)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                force_camera_ratio
                    .after(Transform2System::TransformPropagate)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

fn force_camera_ratio_setup(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(RATIO_BAR_SIZE)),
            color: Color::BLACK,
            ..Default::default()
        },
        transform: Transform::from_xyz(DESIRED_SIZE.x * 0.5 + RATIO_BAR_SIZE * 0.5, 0., 1.),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(RATIO_BAR_SIZE)),
            color: Color::BLACK,
            ..Default::default()
        },
        transform: Transform::from_xyz(DESIRED_SIZE.x * -0.5 - RATIO_BAR_SIZE * 0.5, 0., 1.),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(RATIO_BAR_SIZE)),
            color: Color::BLACK,
            ..Default::default()
        },
        transform: Transform::from_xyz(0., DESIRED_SIZE.y * 0.5 + RATIO_BAR_SIZE * 0.5, 1.),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(RATIO_BAR_SIZE)),
            color: Color::BLACK,
            ..Default::default()
        },
        transform: Transform::from_xyz(0., DESIRED_SIZE.y * -0.5 - RATIO_BAR_SIZE * 0.5, 1.),
        ..Default::default()
    });
}

fn force_camera_ratio(windows: Res<Windows>, mut query: Query<&mut Transform, With<Camera>>) {
    if let Some(window) = windows.get_primary() {
        for mut transform in query.iter_mut() {
            let ratio = window.width() / window.height();
            let mut desired_width = 1920.;
            let mut desired_height = 1080.;
            if ratio > DESIRED_RATIO {
                desired_width *= ratio / DESIRED_RATIO;
            } else {
                desired_height *= DESIRED_RATIO / ratio;
            }
            transform.scale.x = desired_width / window.width();
            transform.scale.y = desired_height / window.height();
        }
    }
}
