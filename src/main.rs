mod board;
mod pieces;
mod game;

use bevy::{picking::hover::PickingInteraction, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{
    board::BoardCoordinates,
    pieces::{PieceColor, PieceType, chess_piece_factory},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .init_resource::<game::GameState>()
        .add_systems(Startup, (setup, board::setup))
        .add_systems(Update, hover_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, game_state: Res<game::GameState>) {
    commands.spawn(Camera2d);

    println!("{:?}", game_state.white_pieces())
    println!("{:?}", game_state.black_pieces())

    // for i in 0..8 {
    //     commands.spawn(chess_piece_factory(
    //         PieceType::Pawn,
    //         PieceColor::White,
    //         BoardCoordinates { col: i, row: 1 },
    //         &asset_server,
    //     ));
    //     commands.spawn(chess_piece_factory(
    //         PieceType::Pawn,
    //         PieceColor::Black,
    //         BoardCoordinates { col: i, row: 6 },
    //         &asset_server,
    //     ));
    // }
    // commands.spawn(chess_piece_factory(
    //     PieceType::Rook,
    //     PieceColor::White,
    //     BoardCoordinates { col: 0, row: 0 },
    //     &asset_server,
    // ));
    // commands.spawn(chess_piece_factory(
    //     PieceType::Rook,
    //     PieceColor::Black,
    //     BoardCoordinates { col: 0, row: 7 },
    //     &asset_server,
    // ));
    // commands.spawn(chess_piece_factory(
    //     PieceType::Rook,
    //     PieceColor::White,
    //     BoardCoordinates { col: 7, row: 0 },
    //     &asset_server,
    // ));
    // commands.spawn(chess_piece_factory(
    //     PieceType::Rook,
    //     PieceColor::Black,
    //     BoardCoordinates { col: 7, row: 7 },
    //     &asset_server,
    // ));
    // commands.spawn(chess_piece_factory(
    //     PieceType::Knight,
    //     PieceColor::White,
    //     BoardCoordinates { col: 1, row: 0 },
    //     &asset_server,
    // ));
    // commands.spawn(chess_piece_factory(
    //     PieceType::Knight,
    //     PieceColor::Black,
    //     BoardCoordinates { col: 1, row: 7 },
    //     &asset_server,
    // ));
    // commands.spawn(chess_piece_factory(
    //     PieceType::Knight,
    //     PieceColor::White,
    //     BoardCoordinates { col: 6, row: 0 },
    //     &asset_server,
    // ));
    // commands.spawn(chess_piece_factory(
    //     PieceType::Knight,
    //     PieceColor::Black,
    //     BoardCoordinates { col: 6, row: 7 },
    //     &asset_server,
    // ));
    // commands.spawn(chess_piece_factory(
    //     PieceType::Bishop,
    //     PieceColor::White,
    //     BoardCoordinates { col: 2, row: 0 },
    //     &asset_server,
    // ));
    // commands.spawn(chess_piece_factory(
    //     PieceType::Bishop,
    //     PieceColor::Black,
    //     BoardCoordinates { col: 2, row: 7 },
    //     &asset_server,
    // ));
    // commands.spawn(chess_piece_factory(
    //     PieceType::Bishop,
    //     PieceColor::White,
    //     BoardCoordinates { col: 5, row: 0 },
    //     &asset_server,
    // ));
    // commands.spawn(chess_piece_factory(
    //     PieceType::Bishop,
    //     PieceColor::Black,
    //     BoardCoordinates { col: 5, row: 7 },
    //     &asset_server,
    // ));
    // commands.spawn(chess_piece_factory(
    //     PieceType::Queen,
    //     PieceColor::White,
    //     BoardCoordinates { col: 4, row: 0 },
    //     &asset_server,
    // ));
    // commands.spawn(chess_piece_factory(
    //     PieceType::Queen,
    //     PieceColor::Black,
    //     BoardCoordinates { col: 4, row: 7 },
    //     &asset_server,
    // ));
    // commands.spawn(chess_piece_factory(
    //     PieceType::King,
    //     PieceColor::White,
    //     BoardCoordinates { col: 3, row: 0 },
    //     &asset_server,
    // ));
    // commands.spawn(chess_piece_factory(
    //     PieceType::King,
    //     PieceColor::Black,
    //     BoardCoordinates { col: 3, row: 7 },
    //     &asset_server,
    // ));
}

fn hover_system(
    mut query: Query<
        (Entity, &PickingInteraction, &mut Sprite, &BoardCoordinates),
        Changed<PickingInteraction>,
    >,
) {
    for (entity, interaction, mut sprite, coordinates) in &mut query {
        match interaction {
            PickingInteraction::Hovered => {
                println!("Hovering over entity: {:?} at {}", entity, coordinates);
                sprite.color = Color::linear_rgb(1.0, 0.0, 0.0);
            }
            PickingInteraction::None => {
                println!("Stopped hovering over: {:?}", entity);
                sprite.color = Color::WHITE;
            }
            PickingInteraction::Pressed => {}
        }
    }
}
