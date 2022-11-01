use std::collections::VecDeque;
use std::ops::Rem;
use std::time::Instant;
use rand::{Rng, SeedableRng};
use neo_granseal::{start, GransealGameConfig, NeoGransealEventHandler, core::NGCore, events::Event, shape_pipeline::ShapeGfx};
use neo_granseal::core::NGCommand;
use neo_granseal::events::{Key, KeyState};
use neo_granseal::shape_pipeline::{FillStyle};
use neo_granseal::util::{Color, Point};

fn main() {
    start(Game {
        title: "Other example".to_string(),
        entities: vec![],
        rng: rand_xorshift::XorShiftRng::from_rng(rand::thread_rng()).expect("Get Rng"),
        points: vec![],
        queue: VecDeque::new(),
        timer: Instant::now(),
        center: Point::new(0.0,0.0),
    },
          GransealGameConfig::new()
              .vsync(false)
              .clear_color(Color::rgb_u8(5,5,12))
    )
}

pub struct Entity {
    pub pos: Point,
    pub center: Point,
    pub radius: f32,
    pub angle: f32,
    pub speed: f32,
}
impl Entity {
    fn update(&mut self,d: std::time::Duration) {
        self.angle += self.speed * d.as_secs_f32();
        self.pos.x = self.center.x + self.angle.cos() * self.radius;
        self.pos.y = self.center.y + self.angle.sin() * self.radius;
    }
}
struct Game {
    title: String,
    rng: rand_xorshift::XorShiftRng,
    entities: Vec<Entity>,
    points: Vec<Point>,
    queue: VecDeque<Point>,
    timer: Instant,
    center: Point,
}
fn grid(g: &mut ShapeGfx, screen_size: Point, grid_size: Point) {
    for x in 0..((screen_size.x/ grid_size.x).floor() as i32){
        let px = x as f32 * grid_size.x;
        for y in 0..((screen_size.y/ grid_size.y).floor() as i32) {
            let py = y as f32 * grid_size.y;
            g.line(Point::new(0.0,py),Point::new(screen_size.x,py));
        }
        g.line(Point::new(px,0.0),Point::new(px,screen_size.y));
    }
}
impl NeoGransealEventHandler for Game {
    fn event(&mut self, core: &mut NGCore, e: Event) {
        match e {
            Event::KeyEvent { key , state: KeyState::Pressed } => {
                if key == Key::F1 { core.cmd(NGCommand::SetCursorVisibility(false));}
                if key == Key::F2 {
                    core.cmd(NGCommand::SetTitle(format!("{}: {}",self.title.as_str(),core.state.fps)));
                }
            }
            Event::Draw => {
                use FillStyle::*;
                let tau = std::f32::consts::TAU;
                let q = tau / 4.0;
                let width = core.config.width as f32;
                let height = core.config.height as f32;
                let time = core.timer.elapsed().as_secs_f32();
                let size = Point::new(64.0,64.0);
                let mut g = ShapeGfx::new(core);
                g.set_fill(true);
                self.entities.iter().for_each(|e|{
                    g.set_fill_style(FadeLeft(Color::DARK_SALMON, Color::GOLD));
                    g.set_line_thickness(8.0);
                    g.line(e.center, e.pos);
                });
                g.set_line_thickness(1.0);
                g.set_fill_style(Solid(Color::DIM_GRAY));
                let c1 = Color::SADDLE_BROWN;
                let c2 = Color::new(0.5451, 0.5, 0.2,0.5 * time.rem(6.28).sin());
                grid(&mut g, Point::new(width, height), Point::new(32.0, 32.0));
                g.set_line_thickness(4.0);
                let cx = self.center.x - 256.0;
                let cy = self.center.y;
                g.set_fill_style(FadeLeft(c2, c1));
                g.arc(Point::new(cx, cy), size, q, q*2.0, 4.0);
                g.set_fill_style(FadeLeft(c1, c2));
                g.rect(Point::new(cx-32.0, cy-32.0), Point::new(64.0, 64.0));
                g.set_fill_style(FadeLeft(c2, c1));
                g.arc(Point::new(cx, cy - 64.0), size, q*2.0, q*3.0, 4.0);
                g.set_fill_style(FadeDown(c1, c2));
                g.rect(Point::new(cx+32.0, cy+32.0), Point::new(64.0, 64.0));
                g.set_fill_style(FadeLeft(c2, c1));
                g.arc(Point::new(cx + 64.0, cy - 64.0), size, q*3.0, q*4.0, 4.0);
                g.set_fill_style(FadeLeft(c2, c1));
                g.rect(Point::new(cx+96.0, cy-32.0), Point::new(64.0, 64.0));
                g.set_fill_style(FadeLeft(c2, c1));
                g.arc(Point::new(cx + 64.0, cy), size, 0.0, q, 4.0);
                g.set_fill_style(FadeDown(c2, c1));
                g.rect(Point::new(cx+32.0, cy-96.0), Point::new(64.0, 64.0));

                let cx = self.center.x - 32.0;
                let cy = self.center.y;
                g.set_fill_style(FadeLeft(c1, c2));
                //gfx.arc(Point::new(cx,cy),64.0,90.0,180.0,4.0);
                g.rect(Point::new(cx-32.0, cy-32.0), Point::new(64.0, 64.0));
                g.set_fill_style(FadeLeft(c1, c2));
                g.rect(Point::new(cx-32.0, cy-96.0), Point::new(64.0, 64.0));
                g.set_fill_style(FadeLeft(c2, c1));
                g.arc(Point::new(cx, cy + 0.0), size, q, q*2.0, 4.0);
                g.set_fill_style(FadeDown(c1, c2));
                g.rect(Point::new(cx+32.0, cy+32.0), Point::new(64.0, 64.0));

                let cx = self.center.x + 128.0;
                let cy = self.center.y;
                g.set_fill_style(FadeLeft(c2, c1));
                g.arc(Point::new(cx, cy), size, q, q*2.0, 4.0);
                g.set_fill_style(FadeLeft(c1, c2));
                g.rect(Point::new(cx-32.0, cy-32.0), Point::new(64.0, 64.0));
                g.set_fill_style(FadeLeft(c2, c1));
                g.arc(Point::new(cx, cy - 64.0), size, q*2.0, q*3.0, 4.0);
                g.set_fill_style(FadeDown(c1, c2));
                g.rect(Point::new(cx+32.0, cy+32.0), Point::new(64.0, 64.0));
                g.set_fill_style(FadeLeft(c2, c1));
                g.arc(Point::new(cx + 64.0, cy - 64.0), size, q*3.0, q*4.0, 4.0);
                g.set_fill_style(FadeLeft(c2, c1));
                //gfx.rect(Point::new(cx+96.0,cy-32.0),Point::new(64.0,64.0));
                g.set_fill_style(FadeLeft(c2, c1));
                g.arc(Point::new(cx + 64.0, cy), size, 0.0, q, 4.0);
                g.set_fill_style(FadeDown(c2, c1));
                g.rect(Point::new(cx+32.0, cy-96.0), Point::new(64.0, 64.0));

                g.set_line_thickness(16.0);
                g.set_fill_style(FadeLeft(Color::DARK_KHAKI, Color::SLATE_BLUE));


                g.circle(Point::new(300.0,300.0),Point::new(200.0,100.0),8.0);

                g.finish();
            }
            Event::Update(d) => {
                if core.state.mouse.left { self.center = core.state.mouse.pos }
                if self.timer.elapsed().as_secs_f32() > 0.005 {
                    self.queue.get_mut(0).unwrap().y = core.timer.elapsed().as_secs_f32().rem(3.14).sin() * 500.0 + self.rng.gen::<f32>() * 100.0;
                    self.queue.rotate_right(1);
                    self.timer = Instant::now();
                }
                self.entities.iter_mut().for_each(|e| e.update(d));
            }
            Event::Load => {
                (0..100).for_each(|_|{
                    self.points.push(
                        Point::new(
                            self.rng.gen::<f32>() * core.config.width as f32,
                            self.rng.gen::<f32>()*core.config.height as f32
                        )
                    );
                });
                (0..core.config.width).for_each(|i| {
                   self.queue.push_back(Point::new(i as f32,self.rng.gen::<f32>() * 400.0));
                });
                self.entities.push(Entity {
                    pos: Point::new(0.0, 0.0),
                    center: Point::new(core.config.width as f32 / 2.0, core.config.height as f32 / 2.0),
                    radius: 256.0,
                    angle: 0.0,
                    speed: 1.0,
                });
                self.entities.push(Entity {
                    pos: Point::new(0.0, 0.0),
                    center: Point::new(core.config.width as f32 / 2.0, core.config.height as f32 / 2.0),
                    radius: 256.0,
                    angle: std::f32::consts::PI,
                    speed: 1.0,
                });
            }
            _ => {}
        }
    }
}