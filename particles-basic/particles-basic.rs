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

        particle.position = pt2(random_range(-200.0, 500.0), random_range(-200.0, 500.0));
        particles.push(particle);

        i += 1;
    }

    // Construct and return the model with our initialised ball
    Model { particles }
}

// By default, `update` is called right before `view` is called each frame.
fn update(app: &App, model: &mut Model, _update: Update) {
    // The "time" variable, created by measuring frame count
    //the loop is going to be 200 frames long
    let frac = (app.elapsed_frames() % 200) as f32 / (200.0);

    // Move the particles
    // Note that we have to create a mutable variable here (&mut)
    for particle in &mut model.particles {
        // Rotate in a circle using sin + cos (and the time from above)
        let movement = random_range(0.1, 2.0) * (frac * TAU).sin();
        let movement_left = random_range(0.1, 2.0) * (frac * TAU).cos();

        // Mutate position of particle
        particle.position = pt2(
            particle.position.x + movement,
            particle.position.y + movement_left,
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
