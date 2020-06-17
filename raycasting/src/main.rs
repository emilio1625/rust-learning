use nannou::prelude::*;
struct Model {
    walls: Vec<Boundary>,
    rays: RaySource,
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run()
}

fn model(_app: &App) -> Model {
    Model {
        walls: vec![Boundary {
            start: Point2 { x: 50., y: 50. },
            end: Point2 { x: 50., y: -50. },
        }],
        rays: RaySource::new(72, Point2 { x: 0., y: 0. }),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.rays.pos = app.mouse.position();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let canvas = app.draw();

    let win = app.window_rect();
    let r = Rect::from_w_h(100., 100.).top_left_of(win);

    canvas.background().color(BLACK);
    canvas.rect().xy(r.xy()).wh(r.wh()).color(WHITE);

    model.rays.show(50., &canvas);
    // dibujamos los rayos
    model.rays.cast_all(&model.walls, &canvas);
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
            .color(WHITE);
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

    fn cast_all(&self, walls: &[Boundary], canvas: &Draw) {
        for ray in &self.rays {
            for wall in walls {
                if let Some(p) = RaySource::cast_ray(ray, &self.pos, wall) {
                    canvas.line().start(self.pos).end(p).weight(1.).color(WHITE);
                }
            }
        }
    }
}
