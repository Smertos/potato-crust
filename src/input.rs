use bevy::{
    app::AppExit,
    input::keyboard::KeyboardInput,
    prelude::*,
    window::{CursorGrabMode, WindowFocused},
};
use num_enum::IntoPrimitive;
use std::ops::Deref;

#[allow(dead_code)]
#[derive(IntoPrimitive)]
#[repr(u32)]
pub enum ScanKey {
    Escape = 1,
    Backspace = 14,
    Q = 16,
    W = 17,
    E = 18,
    Ctrl = 29,
    A = 30,
    S = 31,
    D = 32,
    LShift = 42,
    Alt = 56,
    Space = 57,
}

impl From<ScanKey> for ScanCode {
    fn from(value: ScanKey) -> Self {
        Self(value.into())
    }
}

#[allow(dead_code)]
pub fn keyboard_input(keys: Res<Input<ScanCode>>, mut event_writer: EventWriter<AppExit>) {
    if keys.just_released(ScanKey::Escape.into()) {
        debug!("bye!");

        event_writer.send(AppExit);
    }
}

#[allow(dead_code)]
pub fn keyscan_input(mut key_event_reader: EventReader<KeyboardInput>) {
    use bevy::input::ButtonState;

    for event in key_event_reader.iter() {
        if event.state == ButtonState::Released {
            debug!("scan code: {}", event.scan_code);
        }
    }
}

#[derive(Default)]
pub struct IsGrabbed(bool);

pub fn mouse_grab_input(
    mut windows: ResMut<Windows>,
    keys: Res<Input<ScanCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut event_render: EventReader<WindowFocused>,
    mut is_grabbed: Local<IsGrabbed>,
) {
    let window = windows.get_primary_mut().unwrap();

    if is_grabbed.deref().0 && keys.pressed(ScanKey::Alt.into()) {
        window.set_cursor_grab_mode(CursorGrabMode::None);
        window.set_cursor_visibility(true);

        *is_grabbed = IsGrabbed(false);

        return;
    }

    for event in event_render.iter() {
        if event.focused {
            window.set_cursor_grab_mode(CursorGrabMode::Confined);
            window.set_cursor_visibility(false);

            *is_grabbed = IsGrabbed(true);
        } else if keys.just_released(ScanKey::Escape.into()) {
            window.set_cursor_grab_mode(CursorGrabMode::None);
            window.set_cursor_visibility(true);

            *is_grabbed = IsGrabbed(false);
        }
    }

    if !is_grabbed.deref().0 && mouse_buttons.just_pressed(MouseButton::Left) {
        window.set_cursor_grab_mode(CursorGrabMode::Confined);
        window.set_cursor_visibility(false);

        *is_grabbed = IsGrabbed(true);
    }
}
