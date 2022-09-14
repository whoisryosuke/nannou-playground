use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(_app: &App, _model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = _app.draw();

    let win = _app.window_rect();

    let color_select = map_range(_app.mouse.y, win.top(), win.bottom(), 0.0, 5.0) as i32;

    let bg_color = match color_select {
        0 => RED,
        1 => ORANGE,
        2 => YELLOW,
        3 => GREEN,
        4 => BLUE,
        _ => BLACK,
    };

    draw.background().color(bg_color);

    if _app.mouse.x < 0.0 {
        draw.ellipse().color(STEELBLUE);
    } else {
        draw.ellipse().color(SEAGREEN);
    }

    // Draw to the window frame.
    draw.to_frame(_app, &frame).unwrap();
}
