mod bitboard;
mod board;
mod game;
mod pieces;

use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{bitboard::BitBoard, board::BoardCoordinates, pieces::ChessPiece};

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
            commands.spawn(ChessPiece::new(
                piece_type.clone(),
                piece_color.clone(),
                BoardCoordinates::from_bit(bit),
                &asset_server,
            ));
        }
    }
}
