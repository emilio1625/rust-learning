use nannou::prelude::*;
struct Model {
    walls: Vec<Boundary>,
    rays: RaySource,
}

fn main() {
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> Model {
    // App config
    app.new_window().resized(window_resized).build().unwrap();

    let mut walls = Vec::new();
    for _ in 0..random_range(5, 10) {
        walls.push(Boundary {
            start: Point2 {
                x: random_range(-500., 500.),
                y: random_range(-500., 500.),
            },
            end: Point2 {
                x: random_range(-500., 500.),
                y: random_range(-500., 500.),
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
        rays: RaySource::new(360, Point2 { x: 0., y: 0. }),
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
            x: dim.x / 2.,
            y: -dim.y / 2.,
        },
    });
    model.walls.push(Boundary {
        start: Point2 {
            x: dim.x / 2.,
            y: -dim.y / 2.,
        },
        end: Point2 {
            x: dim.x / 2.,
            y: dim.y / 2.,
        },
    });
    model.walls.push(Boundary {
        start: Point2 {
            x: dim.x / 2.,
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

fn update(app: &App, model: &mut Model, _update: Update) {
    model.rays.pos = app.mouse.position();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let canvas = app.draw();

    canvas.background().color(BLACK);

    // model.rays.show(50., &canvas);
    // dibujamos los rayos
    model.rays.cast_all(&model.walls, rgba(1., 1., 1., 0.2), &canvas);
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
    pos: Point2,
    rays: Vec<Vector2>,
}

impl RaySource {
    fn new(count: i32, pos: Point2) -> Self {
        let mut rays = Vec::new();
        for i in (0..360).step_by(360 / count as usize) {
            let i = i as f32;
            rays.push(Vector2::from_angle(i.to_radians()));
        }
        RaySource { pos, rays }
    }

    fn show(&self, length: f32, canvas: &Draw) {
        for ray in &self.rays {
            canvas
                .arrow()
                .start(self.pos)
                .end(self.pos + *ray * length)
                .weight(1.0)
                .color(STEELBLUE);
        }
    }

    fn cast_ray(dir: &Vector2, pos: &Vector2, wall: &Boundary) -> Option<Point2> {
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
        for ray in &self.rays {
            let mut closest = Vector2::max_value();
            let mut min = std::f32::INFINITY;
            for wall in walls {
                if let Some(p) = RaySource::cast_ray(ray, &self.pos, wall) {
                    let d = self.pos.distance(p);
                    if d < min {
                        closest = p;
                        min = d;
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
