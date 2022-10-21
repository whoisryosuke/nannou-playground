mod particle;
use crate::particle::Particle;
use nannou::noise::*;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

const NUM_PARTICLES: i32 = 100;
const COLORS: [rgb::Rgb<nannou::color::encoding::Srgb, u8>; 6] =
    [BLUE, DARKBLUE, DARKSLATEBLUE, DARKCYAN, CYAN, SLATEBLUE];

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    resolution: u32,
    scale: f32,
    rotation: f32,
    color: Srgb<u8>,
    position: Vec2,
}

struct Model {
    // Debug UI
    settings: Settings,
    egui: Egui,
    // Particles
    particles: Vec<Particle>,
    // Noise gen
    noise: Perlin,
}

fn model(app: &App) -> Model {
    // Create window
    let window_id = app
        .new_window()
        .size(800, 800)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    // Generate particles
    let mut particles: Vec<Particle> = Vec::new();

    for i in 0..NUM_PARTICLES {
        // while i < NUM_PARTICLES {
        // Use index to pick a random color from array
        // We convert the i32 to usize (array index) by using `.try_into().unwrap()` and typing the `let` with `usize`
        let particle_color_id: usize = (i % 6).try_into().unwrap();

        // Create a new particle
        let mut particle = Particle::new(COLORS[particle_color_id]);

        particle.position = pt2(random_range(-400.0, 400.0), random_range(-400.0, 400.0));
        particle.radius = random_range(5.0, 100.0);
        particles.push(particle);
    }

    // Noise generator
    let noise = Perlin::new();

    Model {
        egui,
        settings: Settings {
            resolution: 10,
            scale: 200.0,
            rotation: 0.0,
            color: WHITE,
            position: vec2(0.0, 0.0),
        },
        particles,
        noise,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        // Resolution slider
        ui.label("Resolution:");
        ui.add(egui::Slider::new(&mut settings.resolution, 1..=40));

        // Scale slider
        ui.label("Scale:");
        ui.add(egui::Slider::new(&mut settings.scale, 0.0..=1000.0));

        // Rotation slider
        ui.label("Rotation:");
        ui.add(egui::Slider::new(&mut settings.rotation, 0.0..=360.0));

        // Random color button
        let clicked = ui.button("Random color").clicked();

        if clicked {
            settings.color = rgb(random(), random(), random());
        }
    });

    // Move the particles
    // Note that we have to create a mutable variable here (&mut)
    for particle in &mut model.particles {
        // let movement = random_range(-0.5, 0.5);
        // let movement_left = random_range(-0.5, 0.5);
        let scale = 0.001;
        let movement = model.noise.get([
            scale * particle.position.x.to_f64().unwrap(),
            scale * particle.position.y.to_f64().unwrap(),
            0.0,
        ]);
        let movement_left = model.noise.get([
            scale * particle.position.x.to_f64().unwrap(),
            scale * particle.position.y.to_f64().unwrap(),
            1.0,
        ]);
        // Mutate position of particle
        particle.position += vec2(movement.to_f32().unwrap(), movement_left.to_f32().unwrap());
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let settings = &model.settings;

    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(srgb(0.0, 0.05, 0.1));
    }
    draw.rect()
        .w_h(800.0, 800.0)
        .color(srgba(0.0, 0.05, 0.1, 0.01));

    // Draw our particles
    for particle in &model.particles {
        particle.display(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
