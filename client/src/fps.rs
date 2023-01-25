use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use crate::assets::UiFont;
use crate::states::GameState;

#[derive(Component)]
struct FpsCounterLabel;

pub struct GameFpsCounterPlugin;

impl Plugin for GameFpsCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_fps_counter))
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(update_fps_counter),
            );
    }
}

pub fn setup_fps_counter(mut commands: Commands, ui_font: Res<UiFont>) {
    commands.spawn((
        FpsCounterLabel,
        TextBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: ui_font.0.clone(),
                    font_size: 14.0,
                    ..Default::default()
                },
            ),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
    ));
}

fn update_fps_counter(
    diagnostics: Res<Diagnostics>,
    mut query: Query<(&mut Text, With<FpsCounterLabel>)>,
) {
    let diagnostic = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|x| x.smoothed());

    let Some(fps) = diagnostic else {
        return;
    };

    for (mut text, _) in query.iter_mut() {
        let new_text = format!("FPS: {:.2}", fps);

        if let Some(mut section) = text.sections.first_mut() {
            section.value = new_text;
        }
    }
}
