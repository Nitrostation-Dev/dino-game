mod plugins;
mod components;

use crate::plugins::ground::GroundPlugin;

use bevy::{prelude::{App, PluginGroup, Commands, Camera2dBundle, ClearColor, Color}, DefaultPlugins, window::{WindowPlugin, WindowDescriptor}};
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Dino Game".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(WorldInspectorPlugin::default())
        .add_startup_system(setup_game)
        .add_plugin(GroundPlugin)
        .run();
}

fn setup_game(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default());
}