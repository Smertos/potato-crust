use bevy::prelude::*;
use crate::camera::CameraController;

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    camera3d_bundle: Camera3dBundle,
    controller: CameraController,
    player_marker: Player,
}

impl PlayerBundle {
    pub fn spawn_at_test_position() -> Self {
        Self::spawn_at(Vec3::new(50.0, 15.0, 50.0))
    }

    pub fn spawn_at(position: Vec3) -> Self {
        Self::spawn_looking_at(position, Vec3::new(0.0, 0.0, 0.0))
    }

    pub fn spawn_looking_at(position: Vec3, look_at_target: Vec3) -> Self {
        Self {
            camera3d_bundle: Camera3dBundle {
                transform: Transform::from_translation(position).looking_at(look_at_target, Vec3::Y),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
