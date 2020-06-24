use nannou::prelude::*;
struct Model {
    walls: Vec<Boundary>,
    rays: RaySource,
    vel: Vector2<f32>,
}

fn main() {
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> Model {
    // App config
    app.new_window().resized(window_resized).build().unwrap();

    let mut walls = Vec::new();
    for _ in 0..random_range(3, 7) {
        walls.push(Boundary {
            start: Point2 {
                x: random_range(-500., 0.),
                y: random_range(-500., 0.),
            },
            end: Point2 {
                x: random_range(-500., 0.),
                y: random_range(-500., 0.),
            },
        });
    }
    //the last 4 values are the boundaries of the window
    for _ in 0..4 {
        walls.push(Boundary {
            start: Vector2::zero(),
            end: Vector2::zero(),
        });
    }
    Model {
        walls,
        rays: RaySource::new(60),
        vel: Vector2::zero(),
    }
}

fn window_resized(_app: &App, model: &mut Model, dim: Vector2) {
    for _ in 0..4 {
        model.walls.pop();
    }

    model.walls.push(Boundary {
        start: Point2 {
            x: -dim.x / 2.,
            y: -dim.y / 2.,
        },
        end: Point2 {
            // x: dim.x / 2.,
            x: 0.,
            y: -dim.y / 2.,
        },
    });
    model.walls.push(Boundary {
        start: Point2 {
            // x: dim.x / 2.,
            x: 0.,
            y: -dim.y / 2.,
        },
        end: Point2 {
            // x: dim.x / 2.,
            x: 0.,
            y: dim.y / 2.,
        },
    });
    model.walls.push(Boundary {
        start: Point2 {
            // x: dim.x / 2.,
            x: 0.,
            y: dim.y / 2.,
        },
        end: Point2 {
            x: -dim.x / 2.,
            y: dim.y / 2.,
        },
    });
    model.walls.push(Boundary {
        start: Point2 {
            x: -dim.x / 2.,
            y: dim.y / 2.,
        },
        end: Point2 {
            x: -dim.x / 2.,
            y: -dim.y / 2.,
        },
    });
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.rays.dir = (app.mouse.position() - model.rays.pos).normalize();

    // move management
    let mut input = Vector2::zero();
    for key in app.keys.down.iter() {
        match key {
            Key::W | Key::Up => input.y += 1.,
            Key::S | Key::Down => input.y -= 1.,
            Key::D | Key::Right => input.x += 1.,
            Key::A | Key::Left => input.x -= 1.,
            _ => (),
        }
    }

    input = input.normalize();

    if input.is_zero()
        && (-0.01 < model.vel.x
            && model.vel.x < 0.01
            && -0.01 < model.vel.y
            && model.vel.y < 0.01)
    {
        return;
    }

    const FRICTION: f32 = 0.05;
    const ACCEL: f32 = 0.001;
    let dt = update.since_last.as_millis() as f32;
    let mut vel = model.vel + input * (ACCEL * dt);
    vel = vel.lerp(Vector2::zero(), FRICTION);
    println!("vel = {:?}", vel);

    model.rays.pos += vel * dt;
    model.vel = vel;

    let win = app.window_rect();
    model.rays.pos.x = clamp(model.rays.pos.x, win.left(), 0.0);
    model.rays.pos.y = clamp(model.rays.pos.y, win.bottom(), win.top());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let canvas = app.draw();

    canvas.background().color(BLACK);

    // model.rays.show(50., &canvas);
    // dibujamos los rayos
    model
        .rays
        .cast_all(&model.walls, rgba(1., 1., 1., 1.0), &canvas);
    // dibujamos las paredes
    for wall in &model.walls {
        wall.show(&canvas);
    }

    canvas.to_frame(app, &frame).unwrap();
}

struct Boundary {
    start: Point2,
    end: Point2,
}

impl Boundary {
    fn show(&self, canvas: &Draw) {
        canvas
            .line()
            .start(self.start)
            .end(self.end)
            .weight(1.)
            .color(rgba(1., 1., 1., 0.2));
    }
}

struct RaySource {
    fov: i32,
    pos: Point2,
    dir: Vector2,
}

impl RaySource {
    fn new(fov: i32) -> Self {
        RaySource {
            fov,
            pos: Vector2::zero(),
            dir: Vector2::zero(),
        }
    }

    fn cast_ray(pos: &Vector2, dir: &Vector2, wall: &Boundary) -> Option<Point2> {
        let (x1, y1) = wall.start.into();
        let (x2, y2) = wall.end.into();
        let (x3, y3) = (*pos).into();
        let (x4, y4) = (*pos + *dir).into();

        let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if den == 0. {
            return None;
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

        if t > 0. && t < 1. && u > 0. {
            Some(Point2 {
                x: x1 + t * (x2 - x1),
                y: y1 + t * (y2 - y1),
            })
        } else {
            None
        }
    }

    fn cast_all(&self, walls: &[Boundary], color: Rgba, canvas: &Draw) {
        for offset in (-self.fov / 2)..(self.fov / 2) {
            let dir = {
                let angle = offset as f32;
                self.dir.rotate(angle.to_radians())
            };
            let mut closest = Vector2::max_value();
            let mut min = std::f32::INFINITY;
            for wall in walls {
                if let Some(point) = RaySource::cast_ray(&self.pos, &dir, wall) {
                    let dist = self.pos.distance(point);
                    if dist < min {
                        closest = point;
                        min = dist;
                    }
                }
            }
            canvas
                .line()
                .start(self.pos)
                .end(closest)
                .weight(1.)
                .color(color);
        }
    }
}
