use nannou::prelude::*;

pub struct Particle {
    pub position: Vec2,
    pub radius: f32,
    color: Srgb<u8>,
}

impl Particle {
    pub fn new(color: Srgb<u8>) -> Self {
        let position = pt2(0.0, 0.0);
        let radius = 100.0;
        Particle {
            position,
            color,
            radius,
        }
    }

    pub fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.radius)
            .color(self.color);
    }
}
