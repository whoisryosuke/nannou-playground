use nannou::prelude::*;

mod particle;
use crate::particle::Particle;

const NUM_PARTICLES: i32 = 10;
const COLORS: [rgb::Rgb<nannou::color::encoding::Srgb, u8>; 6] =
    [GREEN, RED, BLUE, PURPLE, YELLOW, ORANGE];

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    particles: Vec<Particle>,
}

fn model(_app: &App) -> Model {
    let mut particles: Vec<Particle> = Vec::new();
    let mut i = 0;

    while i < NUM_PARTICLES {
        // Use index to pick a random color from array
        // We convert the i32 to usize (array index) by using `.try_into().unwrap()` and typing the `let` with `usize`
        let particle_color_id: usize = (i % 6).try_into().unwrap();

        // Create a new particle
        let mut particle = Particle::new(COLORS[particle_color_id]);

        particle.position = pt2(random_range(0.0, 100.0), random_range(0.0, 100.0));
        particles.push(particle);

        i += 1;
    }

    // Construct and return the model with our initialised ball
    Model { particles }
}

// By default, `update` is called right before `view` is called each frame.
fn update(app: &App, model: &mut Model, _update: Update) {
    //the loop is going to be 200 frames long
    let frac = (app.elapsed_frames() % 200) as f32 / (200.0);

    // model.ball.position = pt2(app.mouse.x, app.mouse.y);

    let movement = 0.2 * (frac * TAU).cos();

    // Move the particles
    for particle in &mut model.particles {
        particle.position = pt2(
            particle.position.x + movement,
            particle.position.y + movement,
        );
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing.
    let draw = app.draw();
    // Draw dark gray for the background
    draw.background().color(DIMGRAY);
    // Draw our particles
    for particle in &model.particles {
        particle.display(&draw);
    }
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
