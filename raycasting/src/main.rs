use nannou::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
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
        rays: RaySource::new(36, Point2 { x: 0., y: 0. }),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.rays.update_pos(app.mouse.position());
    if *model.rays.pos.borrow() != *model.rays.rays[1].pos.borrow() {
        println!("D:");
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let canvas = app.draw();

    let win = app.window_rect();
    let r = Rect::from_w_h(100., 100.).top_left_of(win);

    canvas.background().color(BLACK);
    canvas.rect().xy(r.xy()).wh(r.wh()).color(WHITE);

    model.rays.show(50., &canvas);
    // dibujamos los rayos
    for ray in &model.rays.rays {
        //ray.show(50., &canvas);
        for wall in &model.walls {
            if let Some(p) = ray.cast(&wall) {
                canvas
                    .line()
                    .points(*ray.pos.borrow(), p)
                    .color(WHITE)
                    .weight(1.);
            }
        }
    }
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
    pos: Rc<RefCell<Point2>>,
    rays: Vec<Ray>,
}

impl RaySource {
    fn new(count: i32, pos: Point2) -> Self {
        let pos  = Rc::new(RefCell::new(pos));
        let mut rays = Vec::new();
        for i in (0..360).step_by(360 / count as usize) {
            let i = i as f32;
            rays.push(Ray {
                pos: Rc::clone(&pos),
                dir: Vector2::from_angle(i.to_radians()),
            });
        }
        RaySource{pos, rays}
    }
    fn show(&self, length: f32, canvas: &Draw) {
        for ray in &self.rays {
            ray.show(length, canvas)
        }
    }
    fn update_pos(&self, pos: Vector2) {
        self.pos.replace(pos) ;
    }
}

struct Ray {
    pos: Rc<RefCell<Point2>>,
    dir: Vector2,
}

impl Ray {
    fn cast(&self, wall: &Boundary) -> Option<Point2> {
        let (x1, y1) = wall.start.into();
        let (x2, y2) = wall.end.into();
        let (x3, y3) = (*self.pos.borrow()).into();
        let (x4, y4) = (*self.pos.borrow() + self.dir).into();

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

    fn show(&self, length: f32, canvas: &Draw) {
        canvas
            .arrow()
            .start(*self.pos.borrow())
            .end(*self.pos.borrow() + self.dir * length)
            .weight(1.)
            .color(STEELBLUE);
    }
}
