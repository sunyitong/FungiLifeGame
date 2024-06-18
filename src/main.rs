mod components;
mod systems;
mod init_data;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}
};

use crate::systems::*;
use crate::init_data::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(
                WindowPlugin { 
                    primary_window: Some(Window {
                    title: "Fungi Life Game".into(),
                    name: Some("bevy.app".into()),
                    resolution: (CANVAS_SIZE as f32, CANVAS_SIZE as f32).into(),
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin))
        .add_systems(Startup, (setup, init_restriction).chain())
        .add_systems(Update, (update_fungi,spawn_fungi).chain())
        .run();
}