use rand::{Rng, SeedableRng};
use wgpu::VertexFormat::Float32;
use neo_granseal::{start, GransealGameConfig, VSyncMode, NeoGransealEventHandler, core::NGCore, events::Event, shape_pipeline::SSRGraphics};
use neo_granseal::util::{Color, Point2d};

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
    pub pos: Point2d,
    pub center: Point2d,
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
    points: Vec<Point2d>,
}
impl NeoGransealEventHandler for Game {
    fn event(&mut self, core: &mut NGCore, e: Event) {
        match e {
            Event::KeyEvent { .. } => {}
            Event::MouseButton { .. } => {}
            Event::MouseMoved { .. } => {}
            Event::Draw => {
                let width = core.config.width as f32;
                let height = core.config.height as f32;
                let time = core.timer.elapsed().as_secs_f32();
                let mut gfx = SSRGraphics::new(core);
                gfx.thickness = (time.sin() * 8.0).abs();
                let size = 128.0;
                let half = size / 2.0;
                gfx.color = Color::rgb(1.0, 0.0, 0.0);
                gfx.rect(half, half, size, size);
                gfx.color = Color::rgb(0.0, 1.0, 0.0);
                gfx.rect(half + size, half + size, size, size);
                gfx.color = Color::rgb(0.0, 0.0, 1.0);
                gfx.rect(half + size * 2.0, half + size * 2.0, size, size);
                gfx.color = Color::rgb(1.0, 0.0, 1.0);
                gfx.rect(half + size * 3.0, half + size * 3.0, size, size);
                gfx.color = Color::rgb(0.0, 1.0, 1.0);
                gfx.rect(half + size * 4.0, half + size * 4.0, size, size);
                gfx.color = Color::rgb(1.0, 1.0, 1.0);
                gfx.rect(half + size * 5.0, half + size * 5.0, size, size);
                gfx.color = Color::rgb(0.5,1.0,0.5);

                self.entities.iter().for_each(|e| {
                   gfx.line(e.pos,e.center);
                });
                gfx.fill = false;
                gfx.color = Color::NAVY;

                gfx.poly(&self.points);
                gfx.finish();
            }
            Event::Update(d) => {
                self.entities.iter_mut().for_each(|e| e.update(d));
            }
            Event::Load => {
                (0..10).for_each(|i|{
                    self.points.push(
                        Point2d::new(
                            self.rng.gen::<f32>() * core.config.width as f32,
                            self.rng.gen::<f32>()*core.config.height as f32
                        )
                    );
                });
                self.entities.push(Entity {
                    pos: Point2d::new(0.0,0.0),
                    center: Point2d::new(core.config.width as f32 / 2.0,core.config.height as f32 / 2.0),
                    radius: 256.0,
                    angle: 0.0,
                    speed: 1.0,
                });
                self.entities.push(Entity {
                    pos: Point2d::new(0.0,0.0),
                    center: Point2d::new(core.config.width as f32 / 2.0,core.config.height as f32 / 2.0),
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