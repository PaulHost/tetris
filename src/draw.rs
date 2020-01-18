use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;
use crate::position::Position;
use crate::size::Size;
use crate::constants::BLOCK_SIZE;


pub struct Draw {}

impl Draw {
    pub fn to_coordinate(&self, position: i32) -> f64 {
        (position as f64) * BLOCK_SIZE
    }

    pub fn to_coordinate_u32(&self, position: i32) -> u32 {
        self::to_coordinate(position) as u32
    }

    pub fn draw_block(
        &self,
        context: &Context,
        g: &mut G2d,
        position: &Position,
        color: Color,
    ) {
        rectangle(
            color,
            [
                self::to_coordinate(position.x),
                self::to_coordinate(position.y),
                BLOCK_SIZE,
                BLOCK_SIZE
            ],
            context.transform,
            g,
        );
    }

    pub fn draw_rectangle(
        &self,
        context: &Context,
        g: &mut G2d,
        position: &Position,
        size: &Size,
        color: Color,
    ) {
        rectangle(
            color,
            [
                self::to_coordinate(position.x),
                self::to_coordinate(position.y),
                BLOCK_SIZE * (size.width as f64),
                BLOCK_SIZE * (size.height as f64),
            ],
            context.transform,
            g,
        );
    }
}
