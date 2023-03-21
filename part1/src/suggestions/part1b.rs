use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}
struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame){
    let draw = app.draw();

    draw.background()
        .color(CYAN);

    let circle_color = MAGENTA;

    draw.ellipse()
        .y(app.time.sin() * 200.0 )
        .color(circle_color);

    draw.to_frame(app, &frame).unwrap();
}
