use std::collections::VecDeque;
use std::f32::consts::TAU;
use std::ops::Rem;
use std::time::Instant;
use rand::{Rng, SeedableRng};
use neo_granseal::{start, GransealGameConfig, NeoGransealEventHandler, core::NGCore, events::Event, shape_pipeline::ShapeGfx};
use neo_granseal::core::NGCommand;
use neo_granseal::events::{Key, KeyState};
use neo_granseal::shape_pipeline::{FillStyle, Mesh, MeshGen};
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
                let tau = TAU;
                let q = tau / 4.0;
                let width = core.config.width as f32;
                let height = core.config.height as f32;
                let time = core.timer.elapsed().as_secs_f32();
                let size = Point::new(64.0,64.0);
                let mut g = ShapeGfx::new(core);
                // let mut points: Vec<Point> = vec![];
                // g.set_line_thickness(1.0);
                // (0..500).for_each(|i| {
                //     points.push(Point::new(self.rng.gen::<f32>() * width,self.rng.gen::<f32>() * height));
                // });
                // g.set_fill_style(Solid(Color::new(self.rng.gen(),self.rng.gen(),self.rng.gen(),1.0)));
                // g.poly(&points);
                // g.set_fill(true);
                // self.entities.iter().for_each(|e|{
                //     g.set_fill_style(FadeLeft(Color::DARK_SALMON, Color::GOLD));
                //     g.set_line_thickness(8.0);
                //     g.line(e.center, e.pos);
                // });
                // g.set_line_thickness(1.0);
                // g.set_fill_style(Solid(Color::DIM_GRAY));
                // let c1 = Color::SADDLE_BROWN;
                // let c2 = Color::new(0.5451, 0.5, 0.2,0.5 * time.rem(6.28).sin());
                // grid(&mut g, Point::new(width, height), Point::new(32.0, 32.0));
                // g.set_line_thickness(4.0);
                // let cx = self.center.x - 256.0;
                // let cy = self.center.y;
                // g.set_fill_style(FadeLeft(c2, c1));
                // g.arc(Point::new(cx, cy), size, q, q*2.0, 4.0);
                // g.set_fill_style(FadeLeft(c1, c2));
                // g.rect(Point::new(cx-32.0, cy-32.0), Point::new(64.0, 64.0));
                // g.set_fill_style(FadeLeft(c2, c1));
                // g.arc(Point::new(cx, cy - 64.0), size, q*2.0, q*3.0, 4.0);
                // g.set_fill_style(FadeDown(c2, c1));
                // g.rect(Point::new(cx+32.0, cy+32.0), Point::new(64.0, 64.0));
                // g.set_fill_style(FadeLeft(c2, c1));
                // g.arc(Point::new(cx + 64.0, cy - 64.0), size, q*3.0, q*4.0, 4.0);
                // g.set_fill_style(FadeLeft(c2, c1));
                // g.rect(Point::new(cx+96.0, cy-32.0), Point::new(64.0, 64.0));
                // g.set_fill_style(FadeLeft(c2, c1));
                // g.arc(Point::new(cx + 64.0, cy), size, 0.0, q, 4.0);
                // g.set_fill_style(FadeDown(c1, c2));
                // g.rect(Point::new(cx+32.0, cy-96.0), Point::new(64.0, 64.0));
                //
                // let cx = self.center.x - 32.0;
                // let cy = self.center.y;
                // g.set_fill_style(FadeLeft(c1, c2));
                // //gfx.arc(Point::new(cx,cy),64.0,90.0,180.0,4.0);
                // g.rect(Point::new(cx-32.0, cy-32.0), Point::new(64.0, 64.0));
                // g.set_fill_style(FadeLeft(c1, c2));
                // g.rect(Point::new(cx-32.0, cy-96.0), Point::new(64.0, 64.0));
                // g.set_fill_style(FadeLeft(c2, c1));
                // g.arc(Point::new(cx, cy + 0.0), size, q, q*2.0, 4.0);
                // g.set_fill_style(FadeDown(c2, c1));
                // g.rect(Point::new(cx+32.0, cy+32.0), Point::new(64.0, 64.0));
                //
                // let cx = self.center.x + 128.0;
                // let cy = self.center.y;
                // g.set_fill_style(FadeLeft(c2, c1));
                // g.arc(Point::new(cx, cy), size, q, q*2.0, 4.0);
                // g.set_fill_style(FadeLeft(c1, c2));
                // g.rect(Point::new(cx-32.0, cy-32.0), Point::new(64.0, 64.0));
                // g.set_fill_style(FadeLeft(c2, c1));
                // g.arc(Point::new(cx, cy - 64.0), size, q*2.0, q*3.0, 4.0);
                // g.set_fill_style(FadeDown(c2, c1));
                // g.rect(Point::new(cx+32.0, cy+32.0), Point::new(64.0, 64.0));
                // g.set_fill_style(FadeLeft(c2, c1));
                // g.arc(Point::new(cx + 64.0, cy - 64.0), size, q*3.0, q*4.0, 4.0);
                // g.set_fill_style(FadeLeft(c2, c1));
                // //gfx.rect(Point::new(cx+96.0,cy-32.0),Point::new(64.0,64.0));
                // g.set_fill_style(FadeLeft(c2, c1));
                // g.arc(Point::new(cx + 64.0, cy), size, 0.0, q, 4.0);
                // g.set_fill_style(FadeDown(c1, c2));
                // g.rect(Point::new(cx+32.0, cy-96.0), Point::new(64.0, 64.0));
                //
                // g.set_line_thickness(1.0 + 100.0 * (time).rem(tau).sin().abs());
                // g.set_fill_style(FadeLeft(Color::DARK_KHAKI, Color::SLATE_BLUE));
                // g.f(false);
                //
                // g.circle(Point::new(300.0,300.0),Point::new(200.0,100.0),15.0);
                //
                // let mut m1 = MeshGen::rect_filled(Point::new(0.0, 0.0), Point::new(32.0, 32.0), FadeLeft(Color::SALMON,Color::AQUA));
                // let mut m2 = MeshGen::rect_outlined(Point::new(0.0,64.0), Point::new(32.0,128.0), 8.0,FadeDown(Color::DARK_SLATE_BLUE,Color::SEA_GREEN));
                // let mut m3 = MeshGen::rect_filled(Point::new(64.0,64.0), Point::new(128.0,196.0),Corners(Color::RED,Color::LIME,Color::BLUE,Color::MINT_CREAM));
                // let mut m4 = MeshGen::oval_filled(Point::new(300.0,100.0),Point::new(32.0,192.0),0.0,TAU,8.0,Radial(Color::RED,Color::LIME));
                //
                // let s = MeshGen::combine(vec![m1,m2,m3,m4]);
                //
                // g.draw_mesh(s,Point::new(64.0,400.0));


                let mut pixels: Vec<Mesh> = vec![];
                let size = 64;
                let mut count = 0;
                for x in (0..width  as i32).step_by(size) {
                    for y in (0..height as i32).step_by(size) {
                        count += 1;
                        pixels.push(MeshGen::oval_filled(Point::new(x as f32,y as f32),Point::new(x as f32 + size as f32, y as f32 + size as f32),0.0,TAU,8.0,Solid(Color::rgb(self.rng.gen(),self.rng.gen(),self.rng.gen()))));
                    }
                }
                let m = MeshGen::combine(pixels);

                g.draw_mesh(m,Point::new(0.0,0.0));

                g.finish();
            }
            Event::Update(d) => {
                if core.state.mouse.left { self.center = core.state.mouse.pos }
                if self.timer.elapsed().as_secs_f32() > 0.005 {
                    self.queue.get_mut(0).unwrap().y = core.timer.elapsed().as_secs_f32().rem(6.28).sin() * 500.0 + self.rng.gen::<f32>() * 100.0;
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