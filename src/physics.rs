use graphics::types::Vec2d;

use crate::constants::PHYSICS_SCALE;

pub fn scale_to_screen(pos: Vec2d) -> Vec2d {
    [pos[0] * PHYSICS_SCALE, pos[1] * PHYSICS_SCALE]
}
