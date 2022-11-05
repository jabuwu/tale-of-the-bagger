use std::io::Cursor;

use bevy::{prelude::*, window::WindowId, winit::WinitWindows};
use bevy_kira_audio::AudioPlugin;
use bevy_spine::prelude::*;
use common::CommonPlugins;
use winit::window::Icon;

use crate::{game::GamePlugin, loading::LoadingPlugin};

pub use crate::{app_state::AppState, asset_library::AssetLibrary};

#[cfg(target_os = "ios")]
use bevy::window::WindowMode;

#[cfg(feature = "embedded_assets")]
use common::embedded_assets::EmbeddedAssetIoPlugin;

pub fn game() {
    let mut window_descriptor = WindowDescriptor {
        title: "Tale of the Bagger: A Love Story".to_string(),
        canvas: Some("#bevy".to_owned()),
        fit_canvas_to_parent: true,
        ..Default::default()
    };

    #[cfg(target_os = "ios")]
    {
        window_descriptor.mode = WindowMode::BorderlessFullscreen;
        window_descriptor.resizable = false;
    }

    #[cfg(not(target_os = "ios"))]
    {
        window_descriptor.width = 1440.;
        window_descriptor.height = 810.;
    }

    let mut app = App::new();

    app.insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(window_descriptor)
        .init_resource::<AssetLibrary>()
        .add_state(AppState::default());

    #[cfg(not(feature = "embedded_assets"))]
    app.add_plugins(DefaultPlugins);
    #[cfg(feature = "embedded_assets")]
    app.add_plugins_with(DefaultPlugins, |group| {
        group.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetIoPlugin)
    });

    app.add_plugins(CommonPlugins)
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

#[cfg(target_os = "ios")]
#[bevy_main]
fn main() {
    game();
}

pub mod app_state;
pub mod asset_library;
pub mod common;
pub mod game;
pub mod loading;
