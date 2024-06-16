mod components;
mod systems;
mod init_data;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    app::ScheduleRunnerPlugin,
    utils::Duration
};

use crate::systems::*;
use crate::init_data::*;

fn main() {
    let mut rng = rand::thread_rng();
    App::new()
        .add_plugins((
            ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 30.0,
            )),
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
        .add_systems(Startup, setup)
        .add_systems(Update, update_fungi)
        .add_systems(PostUpdate,spawn_fungi)
        .run();
}