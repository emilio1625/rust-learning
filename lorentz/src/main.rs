use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    points: Vec<Vector3>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model {
        _window,
        points: vec![Vector3 {
            x: 0.01,
            y: 0.0,
            z: 0.0,
        }],
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let dt = 0.01;
    let (mut x, mut y, mut z) = model.points.last().unwrap().to_owned().into();

    const RHO: f32 = 28.;
    const SIGMA: f32 = 10.;
    const BETA: f32 = 8. / 3.;
    let dx = SIGMA * (y - x) * dt;
    let dy = (x * (RHO - z) - y) * dt;
    let dz = (x * y - BETA * z) * dt;

    x += dx;
    y += dy;
    z += dz;

    model.points.push(Vector3 { x, y, z });
    model.points.push(Vector3 { x, y, z }); // twice to draw lines
    println!("model.points.len() = {:?}", model.points.len());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let points = model.points.iter().enumerate().map(|(i, p)| {
        let frac = (i % 1023) as f32;
        (p.to_owned(), hsv(frac / 1023., 1.0, 1.0))
    });

    draw.line_mode()
        .scale(5.)
        .mesh()
        .points_colored(points)
        .z_radians(app.time * 0.33)
        .x_radians(app.time * 0.166 + -app.mouse.y / 100.0)
        .y_radians(app.time * 0.25 + app.mouse.x / 100.0);

    draw.to_frame(app, &frame).unwrap();
}
