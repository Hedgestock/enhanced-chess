mod board;
mod game;
mod pieces;

use bevy::{picking::hover::PickingInteraction, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{
    board::BoardCoordinates,
    game::BitBoard,
    pieces::{ChessPiece, PieceColor, PieceType},
};

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

    println!("{:?}", &game_state.white_pieces());
    println!("{:?}", &game_state.black_pieces());

    for bb in &game_state.pieces {
        let piece_type = &bb.0.0;
        let piece_color = &bb.0.1;
        for bit in get_piece_positions(bb.1.clone()) {
            commands.spawn(ChessPiece::new(
                piece_type.clone(),
                piece_color.clone(),
                BoardCoordinates::from_bit(bit),
                &asset_server,
            ));
        }
    }
}

fn get_piece_positions(mut board: BitBoard) -> Vec<u8> {
    let mut positions = Vec::new();
    while board != 0 {
        // Get index of the lowest set bit (0-63)
        let sq = board.trailing_zeros() as u8;
        positions.push(sq);
        // Clear the lowest set bit
        board &= board - 1;
    }
    positions
}
