use bevy::{prelude::{Plugin, Commands, StartupStage, AssetServer, Res, Name}, sprite::SpriteBundle};

use crate::components::ground::{Ground, Velocity};

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, setup_ground);
    }
}

fn setup_ground(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("ground.png"),
            ..Default::default()
        })
        .insert(Name::new("Ground_0"))
        .insert(Ground)
        .insert(Velocity { x: -1., y: 0. });
}