use neo_granseal::shape_pipeline::{SSRGraphics, SSRRenderData};
use neo_granseal::{
    core::{NGCommand, NGCore},
    events::Event,
    shape_pipeline, start, GransealGameConfig, NeoGransealEventHandler, VSyncMode,
};
use rand::{Rng, SeedableRng};
use neo_granseal::util::{Color, Point};

fn main() {
    start(
        Game::new(),
        GransealGameConfig::new()
            .vsync(VSyncMode::AutoNoVsync)
            .size(128 * 5, 128 * 5),
    );
}
struct Entity {
    pos: Point,
    color: Color,
    rot: f32,
}
struct Game {
    rng: rand_xorshift::XorShiftRng,
    entities: Vec<Entity>,
    size: Point,
    timer: std::time::Instant,
    toggle: bool,
}
impl Game {
    fn new() -> Self {
        Self {
            rng: rand_xorshift::XorShiftRng::from_rng(rand::thread_rng()).expect("Getting Rng."),
            entities: vec![],
            size: Point::new(16.0, 16.0),
            timer: std::time::Instant::now(),
            toggle: true,
        }
    }
}

impl NeoGransealEventHandler for Game {
    fn event(&mut self, core: &mut NGCore, e: Event) {
        match e {
            Event::KeyEvent { .. } => {}
            Event::MouseButton { .. } => {}
            Event::MouseMoved { .. } => {}
            Event::Draw => {
                let mut gfx = SSRGraphics::new(core);
                for e in &self.entities {
                    gfx.color = e.color;
                    gfx.rotation = e.rot;
                    gfx.rect(e.pos, self.size);
                }
                gfx.finish();
            }
            Event::Update(d) => {
                self.entities.iter_mut().for_each(|e|
                    e.rot += d.as_secs_f32() * e.color.r
                );
                if self.timer.elapsed().as_secs_f32() > 1.5 {
                    self.toggle = !self.toggle;
                    self.timer = std::time::Instant::now();
                }
                if self.toggle {
                    self.entities.iter_mut().for_each(|e| {
                        e.color.r = self.rng.gen();
                        e.color.g = self.rng.gen();
                        e.color.b = self.rng.gen();
                    });
                }
            }
            Event::Load => {
                let halfx = self.size.x / 2.0;
                let halfy = self.size.y / 2.0;
                for x in (halfx.floor() as usize..core.config.width as usize).step_by(self.size.x.floor() as usize) {
                    for y in (halfy.floor() as usize..core.config.height as usize).step_by(self.size.y.floor() as usize) {
                        let pos = Point::new(x as f32, y as f32);
                        let color = Color::rgb(self.rng.gen(),self.rng.gen(),self.rng.gen());
                        self.entities.push(Entity {
                            pos,
                            color,
                            rot: 0.0,
                        })
                    }
                }
            }
            Event::Resized(_, _) => {}
            Event::Fps(_) => {}
        }
    }
}
