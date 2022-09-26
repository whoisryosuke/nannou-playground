use nannou::prelude::*;

pub struct Particle {
    pub position: Point2,
    color: Srgb<u8>,
}

impl Particle {
    pub fn new(color: Srgb<u8>) -> Self {
        let position = pt2(0.0, 0.0);
        Particle { position, color }
    }

    pub fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(100.0)
            .color(self.color);
    }
}
