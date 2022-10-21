use nannou::prelude::*;

pub struct Particle {
    pub positions: Vec<Vec2>,
    pub radius: f32,
    color: Srgb<u8>,
}

impl Particle {
    pub fn new(color: Srgb<u8>, initial_position: Vec2) -> Self {
        let mut positions = Vec::new();
        positions.push(initial_position);
        let radius = 100.0;
        Particle {
            positions,
            color,
            radius,
        }
    }
}
