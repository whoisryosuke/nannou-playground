use nannou::prelude::*;

// We start here -- Rust runs this first
// App is created and we use the "model" function below
fn main() {
    nannou::app(model).run();
}

// We'd store app state here if we had any
struct Model;

// Here we create the new nannou app, open window, etc
fn model(app: &App) -> Model {
    app.new_window().event(event).view(view).build().unwrap();
    Model
}

// We capture events in this callback function
fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    // Use Rust's `match` (like JS `switch`) to handle different event types
    match event {
        // Keyboard input
        KeyPressed(_key) => {
            println!("Key pressed {:#?}", _key);
            // Returns
            // Key pressed Key9
            // Key pressed Numpad1
        }
        // Mouse input
        MousePressed(_button) => {
            println!("Mouse pressed {:#?}", _button);
            // Returns
            // Mouse pressed Left
        }
        // Other events, like window resizing
        _other => {}
    }
}

// Here we can "draw" to the screen
fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(DIMGRAY);
}
