use bevy::prelude::Component;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}