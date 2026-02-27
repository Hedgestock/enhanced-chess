mod bitboard;
mod board;
mod game;
mod rendering;

use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::rendering::ChessPiece;

use crate::board::BoardCoordinates;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .init_resource::<game::GameState>()
        .add_systems(Startup, (setup, board::setup))
        // .add_systems(Update, hover_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, game_state: Res<game::GameState>) {
    commands.spawn(Camera2d);

    for bb in &game_state.pieces {
        let piece_type = &bb.0.0;
        let piece_color = &bb.0.1;
        for bit in bb.1.get_piece_positions() {
            commands
                .spawn(ChessPiece::new(
                    piece_type.clone(),
                    piece_color.clone(),
                    BoardCoordinates::from_bit(bit),
                    &asset_server,
                ))
                .observe(on_click_piece)
                .observe(on_drag_piece);
        }
    }
}

fn on_drag_piece(drag: On<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(drag.entity) {
        // Pointer<Drag> provides 'delta' in world space units for 2D sprites
        transform.translation.x += drag.delta.x;
        transform.translation.y -= drag.delta.y; // Y is often inverted in screen-to-world
    }
}

fn on_click_piece(click: On<Pointer<Press>>, mut transforms: Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(click.entity) {
        transform.translation.x = click.hit.position.unwrap().x;
        transform.translation.y = click.hit.position.unwrap().y;
    }
}
