use bevy::prelude::*;

use crate::game_of_life::plugin::GameOfLife;

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut game: ResMut<GameOfLife>
) {
    if keys.just_pressed(KeyCode::Space) {
        game.is_paused = !game.is_paused
    }
}