use std::io::Cursor;

use bevy::{prelude::*, window::WindowId, winit::WinitWindows};
use bevy_kira_audio::AudioPlugin;
use bevy_spine::prelude::*;
use common::CommonPlugins;
use winit::window::Icon;

use crate::{game::GamePlugin, loading::LoadingPlugin};

pub use crate::{app_state::AppState, asset_library::AssetLibrary};

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
        .init_resource::<AssetLibrary>()
        .add_state(AppState::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(CommonPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(SpinePlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(set_window_icon)
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

pub mod app_state;
pub mod asset_library;
pub mod common;
pub mod game;
pub mod loading;
