use rand::{Rng, SeedableRng};
use neo_granseal::{start, GransealGameConfig, VSyncMode, NeoGransealEventHandler, core::NGCore, events::Event, shape_pipeline::SSRGraphics};
use neo_granseal::events::Key::P;
use neo_granseal::shape_pipeline::{FillStyle, LineStyle};
use neo_granseal::util::{Color, Point};

fn main() {
    start(Game {
        entities: vec![],
        rng: rand_xorshift::XorShiftRng::from_rng(rand::thread_rng()).expect("Get Rng"),
        points: vec![]
    },
          GransealGameConfig::new()
              .size(128*6,128*6)
              .vsync(VSyncMode::AutoNoVsync)
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
    rng: rand_xorshift::XorShiftRng,
    entities: Vec<Entity>,
    points: Vec<Point>,
}
impl NeoGransealEventHandler for Game {
    fn event(&mut self, core: &mut NGCore, e: Event) {
        match e {
            Event::KeyEvent { .. } => {}
            Event::MouseButton { .. } => {}
            Event::MouseMoved { .. } => {}
            Event::Draw => {
                use FillStyle::*;
                let width = core.config.width as f32;
                let height = core.config.height as f32;
                let time = core.timer.elapsed().as_secs_f32();
                let mut gfx = SSRGraphics::new(core);
                //gfx.thickness = (time.sin() * 8.0).abs();
                gfx.thickness = 32.0;
                //gfx.rotation = time.sin() * std::f32::consts::PI * 2.0;
                let size = Point::new(128.0, 128.0);
                let halfx = size.x / 2.0;
                let halfy = size.y / 2.0;
                //gfx.fill = if time.sin().abs() > 0.5 {true} else {false};
                gfx.color = FadeDown(Color::RED,Color::NAVY);
                gfx.rect(Point::new(halfx, halfy), size);
                gfx.color = FadeLeft(Color::GREEN,Color::MAGENTA);
                gfx.rect(Point::new(halfx + size.x, halfy + size.y), size);
                gfx.color = Corners(Color::RED,Color::GREEN,Color::BLUE,Color::BLACK);
                gfx.rect(Point::new(halfx + size.x * 2.0, halfy + size.y * 2.0), size);
                //gfx.color = Solid(Color::rgb(1.0, 0.0, 1.0));
                gfx.oval(Point::new(halfx + size.x * 3.0, halfy + size.y * 3.0), size);
                gfx.color = Solid(Color::rgb(0.0, 1.0, 1.0));
                gfx.rect(Point::new(halfx + size.x * 4.0, halfy + size.y * 4.0), size);
                gfx.color = Solid(Color::rgb(1.0, 1.0, 1.0));
                gfx.rect(Point::new(halfx + size.x * 5.0, halfy + size.y * 5.0), size);

                gfx.fill = false;
                gfx.color = FillStyle::Solid(Color::NAVY);
                gfx.poly(&self.points);

                gfx.line_style = LineStyle::Center(false);
                gfx.color = Solid(Color::rgb(0.5,1.0,0.5));
                gfx.color = FadeDown(Color::rgba(0.2,0.2,0.7,0.0),Color::MAROON);
                let mut center = Point::new(0.0,0.0);
                self.entities.iter().for_each(|e| {
                    center = e.center;
                    gfx.line(e.pos,e.center);
                });
                gfx.fill = true;
                gfx.color = Corners(Color::MAGENTA,Color::LIME,Color::BLACK,Color::RED);
                gfx.oval(center,Point::new(8.0,8.0));
                gfx.finish();
            }
            Event::Update(d) => {
                self.entities.iter_mut().for_each(|e| e.update(d));
            }
            Event::Load => {
                (0..10).for_each(|i|{
                    self.points.push(
                        Point::new(
                            self.rng.gen::<f32>() * core.config.width as f32,
                            self.rng.gen::<f32>()*core.config.height as f32
                        )
                    );
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
            Event::Resized(_, _) => {}
            Event::Fps(_) => {}
        }
    }
}