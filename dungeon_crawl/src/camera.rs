use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_y: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(player_position: Point) -> Self {
        Self{
            left_x: player_position.x - DISPLAY_WIDTH/2,
            right_x: player_position.x + DISPLAY_WIDTH/2,
            top_y: player_position.x - DISPLAY_WIDTH/2,
        }
    }
}
