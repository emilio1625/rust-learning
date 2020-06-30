use nannou::prelude::*;
use num_complex::{Complex, Complex32};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let (x, y) = app.window_rect().w_h();
    let scale = x.min(y);
    println!("scale = {:?}", scale);

    const MAX_ITER: u8 = 10;

    let mut c: Complex32 = Complex::new(0., 0.);
    let mut points = Vec::new();

    for x in (-x as i32 / 2)..(x as i32 / 2) {
        for y in (-y as i32 / 2)..(y as i32 / 2) {
            c.re = map_range(x as f32, 0., 200., -2., 2.);
            c.im = map_range(y as f32, 0., 200., -2., 2.);
            let mut z: Complex32 = Complex::new(0., 0.);
            let mut n = 0;
            while n < MAX_ITER && z.norm_sqr() <= 16. {
                z = z * z + c;
                n+=1;
            }

            points.push((
                vec2(
                    c.re, c.im
                ),
                rgb8(n * 10, n * 10, n * 10),
            ));
        }
    }

    draw.scale(scale / 2.)
        .point_mode()
        .mesh()
        .points_colored(points)
        ;

    draw.to_frame(app, &frame).unwrap();
}
