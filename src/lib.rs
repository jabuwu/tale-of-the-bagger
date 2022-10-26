// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Cursor;

use bevy::{prelude::*, window::WindowId, winit::WinitWindows};
use bevy_kira_audio::{Audio, AudioControl, AudioPlugin};
use winit::window::Icon;

pub fn game() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 1440.,
            height: 810.,
            resizable: false,
            title: "Tale of the Bagger: A Love Story".to_string(), // ToDo
            canvas: Some("#bevy".to_owned()),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_startup_system(set_window_icon)
        .add_startup_system(setup)
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon_buf = Cursor::new(include_bytes!("../assets/textures/app_icon.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("textures/background.png"),
        transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(0.75)),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("textures/background_front.png"),
        transform: Transform::from_xyz(0., 0., 0.1).with_scale(Vec3::splat(0.75)),
        ..Default::default()
    });

    audio.play(asset_server.load("audio/ambience.ogg")).looped();
}
