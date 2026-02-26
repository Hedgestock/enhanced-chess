use std::fmt;

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

pub const BOARD_SIZE: u8 = 8;
pub const SQUARE_SIZE: f32 = 50.0;

#[derive(Component)]
pub struct BoardCoordinates {
    pub col: u8,
    pub row: u8,
}

impl fmt::Display for BoardCoordinates {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", (self.col + 97) as char, self.row + 1)
    }
}

pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let black_square = images.add(Image::new_fill(
        Extent3d {
            width: SQUARE_SIZE as u32,
            height: SQUARE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        // Initialize it with the specified color converted to a u8 array
        &LinearRgba::new(0.7, 0.7, 0.7, 1.0).to_u8_array(),
        // Use the standard sRGB format
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    ));

    let white_square = images.add(Image::new_fill(
        Extent3d {
            width: SQUARE_SIZE as u32,
            height: SQUARE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        // Initialize it with the specified color converted to a u8 array
        &LinearRgba::WHITE.to_u8_array(),
        // Use the standard sRGB format
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    ));

    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            let is_dark = (x + y) % 2 == 0;
            let image = if is_dark {
                black_square.clone()
            } else {
                white_square.clone()
            };

            commands
                .spawn((
                    Sprite::from_image(image),
                    Transform::from_xyz(
                        (x as f32 - BOARD_SIZE as f32 / 2.0) * SQUARE_SIZE,
                        (y as f32 - BOARD_SIZE as f32 / 2.0) * SQUARE_SIZE,
                        0.0,
                    ),
                    BoardCoordinates { row: x, col: y },
                    Pickable::default(),
                ))
                .observe(
                    |event: On<Pointer<Over>>,
                     mut query: Query<&mut Sprite, With<BoardCoordinates>>| {
                        println!(
                            "MyEvent was triggered on this specific tile! {:?}",
                            event.entity
                        );
                        if let Ok(mut sprite) = query.get_mut(event.entity) {
                            sprite.color = Color::linear_rgb(1.0, 0.0, 0.0);
                        }
                    },
                )                .observe(
                    |event: On<Pointer<Out>>,
                     mut query: Query<&mut Sprite, With<BoardCoordinates>>| {
                        println!(
                            "MyEvent was triggered on this specific tile! {:?}",
                            event.entity
                        );
                        if let Ok(mut sprite) = query.get_mut(event.entity) {
                            sprite.color = Color::WHITE;
                        }
                    },
                );
        }
    }
}
