use bevy::prelude::*;

use crate::board;

#[derive(PartialEq, Eq, Component)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Component)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Bundle)]
pub struct ChessPiece {
    piece: PieceType,
    color: PieceColor,
    sprite: Sprite,
    transform: Transform,
    pickable: Pickable,
}

pub fn chess_piece_factory(
    piece: PieceType,
    color: PieceColor,
    position: board::BoardCoordinates,
    asset_server: &Res<AssetServer>,
) -> ChessPiece {
    let path = format!(
        "pieces/01_classic/{}-{}.png",
        if color == PieceColor::White { "w" } else { "b" },
        match piece {
            PieceType::Pawn => "pawn",
            PieceType::Knight => "knight",
            PieceType::Bishop => "bishop",
            PieceType::Rook => "rook",
            PieceType::Queen => "queen",
            PieceType::King => "king",
        }
    );

    return ChessPiece {
        piece,
        color,
        sprite: Sprite {
            image: (asset_server.load(path)),
            custom_size: Some(Vec2::new(board::SQUARE_SIZE, board::SQUARE_SIZE)),
            ..default()
        },
        transform: Transform::from_xyz(
            (position.col as f32 - board::BOARD_SIZE as f32 / 2.0) * board::SQUARE_SIZE,
            (position.row as f32 - board::BOARD_SIZE as f32 / 2.0) * board::SQUARE_SIZE,
            1.,
        ),
        pickable: Pickable {
            should_block_lower: false,
            ..default()
        },
    };
}
