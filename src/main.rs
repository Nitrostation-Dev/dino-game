use bevy::{prelude::{App, PluginGroup}, DefaultPlugins, window::{WindowPlugin, WindowDescriptor}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Dino Game".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .run();
}