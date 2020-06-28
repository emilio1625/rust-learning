use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    pendulums: Vec<Pendulum>,
    points: Vec<Point2>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();

    let length = 100.;
    let start = vec2(0., length);
    let angle = PI / 4.;
    let u = vec2(length * angle.sin(), length * angle.cos());
    let end = start - u;
    Model {
        _window,
        pendulums: vec![
            Pendulum {
                mass: 4.,
                angle,
                length,
                vel: 0.,
                pos: start,
                end,
            },
            Pendulum {
                mass: 4.,
                angle:  2.*angle,
                length,
                vel: 0.,
                pos: end,
                end: end - u,
            },
        ],
        points: Vec::new(),
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let m1 = model.pendulums[0].mass;
    let O1 = model.pendulums[0].angle;
    let l1 = model.pendulums[0].length;
    let v1 = model.pendulums[0].vel;
    let m2 = model.pendulums[1].mass;
    let O2 = model.pendulums[1].angle;
    let l2 = model.pendulums[1].length;
    let v2 = model.pendulums[1].vel;
    let dt = update.since_last.as_millis() as f32 / 100.;
    const G: f32 = 9.8;
    const FRICTION: f32 = 0.995;

    let den = 2. * m1 + m2 - m2 * (2. * O1 - 2. * O2).cos();
    let accel = -G * (2. * m1 + m2) * O1.sin()
        - m2 * G * (O1 - 2. * O2).sin()
        - 2. * (O1 - O2).sin() * m2 * (v2 * v2 * l2 + v1 * v1 * l1 * (O1 - O2).cos());

    model.pendulums[0].vel += (accel / (l1 * den)) * dt;
    model.pendulums[0].vel *= FRICTION;
    model.pendulums[0].angle += model.pendulums[0].vel * dt;

    let accel = 2.
        * (O1 - O2).sin()
        * (v1 * v1 * l1 * (m1 + m2)
            + G * (m1 + m2) * O1.cos()
            + v2 * v2 * l2 * m2 * (O1 - O2).cos());

    model.pendulums[1].vel += (accel / (l2 * den)) * dt;
    model.pendulums[1].vel *= FRICTION;
    model.pendulums[1].angle += model.pendulums[1].vel * dt;


    for i in 0..model.pendulums.len() {
        if i == 1 {
            model.points.push(model.pendulums[1].end);
            model.pendulums[1].pos = model.pendulums[0].end;
        }

        let start = model.pendulums[i].pos;
        let length = model.pendulums[i].length;
        let angle = model.pendulums[i].angle;
        let end = start - vec2(length * angle.sin(), length * angle.cos());
        model.pendulums[i].end = end;
    }

    // Update 2nd vector origin
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for pendulum in &model.pendulums {
        let start = pendulum.pos;
        let end = pendulum.end;
        draw.line().start(start).end(end).weight(2.).color(WHITE);

        draw.ellipse().xy(end).radius(pendulum.mass).color(WHITE);
    }

    
    let points = model.points.iter().enumerate().map(|(i, p)| {
        let frac = (i % 1023) as f32;
        (p.to_owned(), hsv(frac / 1023., 1.0, 1.0))
    });

    draw.point_mode().polyline().points_colored(points);

    draw.to_frame(app, &frame).unwrap();
}

struct Pendulum {
    mass: f32,
    length: f32,
    angle: f32,
    vel: f32,
    end: Vector2,
    pos: Vector2,
}
