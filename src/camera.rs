use std::f32::consts::PI;

use bevy::{prelude::*, input::mouse::MouseMotion};

use crate::input::ScanKey;

#[derive(Component)]
pub struct CameraController {
    pub fly_speed: f32,
    pub initialized: bool,
    pub pitch: f32,
    pub sensitivity: f32,
    pub velocity: Vec3,
    pub yaw: f32,
}

impl CameraController {
    pub fn new(fly_speed: f32, sensitivity: f32,) -> Self {
        Self {
            fly_speed,
            initialized: false,
            pitch: 0.0,
            sensitivity,
            velocity: Vec3::ZERO,
            yaw: 0.0,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    let camera_3d_bundle = Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };
    let camera_controller = CameraController::new(4.0, 0.4);

    commands.spawn((
        camera_controller,
        camera_3d_bundle,
    ));
}

pub fn camera_input(
    keys: Res<Input<ScanCode>>,
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    mut query: Query<(&mut CameraController, &mut Transform), With<Camera>>
) {
    let dt = time.delta_seconds();

    for (mut controller, mut transform) in query.iter_mut() {
        if !controller.initialized {
            let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            controller.yaw = yaw;
            controller.pitch = pitch;
            controller.initialized = true;
        }

        let mut axis_input = Vec3::ZERO;
        let mut is_running = false;

        if keys.pressed(ScanKey::W.into()) {
            axis_input.z += 1.0;
        }

        if keys.pressed(ScanKey::S.into()) {
            axis_input.z -= 1.0;
        }

        if keys.pressed(ScanKey::A.into()) {
            axis_input.x -= 1.0;
        }

        if keys.pressed(ScanKey::D.into()) {
            axis_input.x += 1.0;
        }

        if keys.pressed(ScanKey::Space.into()) {
            axis_input.y += 1.0;
        }

        if keys.pressed(ScanKey::Ctrl.into()) {
            axis_input.y -= 1.0;
        }

        if keys.pressed(ScanKey::LShift.into()) {
            is_running = true;
        }

        if axis_input != Vec3::ZERO {
            let max_speed = if is_running { 2.0 * controller.fly_speed } else { controller.fly_speed };

            controller.velocity = axis_input.normalize() * max_speed;
        } else {
            controller.velocity = controller.velocity.lerp(Vec3::ZERO, 0.3);
        }

        let forward = transform.forward();
        let right = transform.right();

        transform.translation += controller.velocity.x * dt * right
            + controller.velocity.y * dt * Vec3::Y
            + controller.velocity.z * dt * forward;

        let mut mouse_delta = Vec2::ZERO;

        for mouse_event in mouse_events.iter() {
            mouse_delta += mouse_event.delta;
        }

        if mouse_delta != Vec2::ZERO {
            controller.pitch = (controller.pitch - mouse_delta.y * 0.5 * controller.sensitivity * dt)
                .clamp(-PI / 2.0, PI / 2.0);
            controller.yaw -= mouse_delta.x * controller.sensitivity * dt;

            transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, controller.yaw, controller.pitch);
        }

        dbg!(transform.translation);
    }
}

#[derive(Default)]
pub struct GameplayCameraPlugin;

impl Plugin for GameplayCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_camera)
            .add_system(camera_input);
    }
}