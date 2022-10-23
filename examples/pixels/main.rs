use neo_granseal::shape_pipeline::{SSRGraphics, SSRRenderData};
use neo_granseal::{
    core::{NGCommand, NGCore},
    events::Event,
    shape_pipeline, start, GransealGameConfig, NeoGransealEventHandler, VSyncMode,
};
use rand::{Rng, SeedableRng};

fn main() {
    start(
        Game::new(),
        GransealGameConfig::new()
            .vsync(VSyncMode::AutoNoVsync)
            .size(128 * 5, 128 * 5),
    );
}
struct Entity {
    x: f32,
    y: f32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
    rot: f32,
}
struct Game {
    rng: rand_xorshift::XorShiftRng,
    gfx: SSRGraphics,
    entities: Vec<Entity>,
    size: usize,
    timer: std::time::Instant,
    toggle: bool,
}
impl Game {
    fn new() -> Self {
        Self {
            rng: rand_xorshift::XorShiftRng::from_rng(rand::thread_rng()).expect("Getting Rng."),
            gfx: SSRGraphics::new(),
            entities: vec![],
            size: 16,
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
                self.gfx.clear();
                for e in &self.entities {
                    self.gfx.color(e.r, e.g, e.b, e.a);
                    self.gfx.set_rotation(e.rot);
                    self.gfx.rect(e.x, e.y, self.size as f32, self.size as f32);
                }
                core.cmd(self.gfx.finish());
            }
            Event::Update(d) => {
                self.entities.iter_mut().for_each(|e|
                    e.rot += d.as_secs_f32() * e.r
                );
                if self.timer.elapsed().as_secs_f32() > 1.5 {
                    self.toggle = !self.toggle;
                    self.timer = std::time::Instant::now();
                }
                if self.toggle {
                    self.entities.iter_mut().for_each(|e| {
                        e.r = self.rng.gen();
                        e.g = self.rng.gen();
                        e.b = self.rng.gen();
                    });
                }
            }
            Event::Load => {
                let step: usize = self.size;
                let half = step / 2;
                for x in (half..core.config.width as usize).step_by(step) {
                    for y in (half..core.config.height as usize).step_by(step) {
                        let x = x as f32;
                        let y = y as f32;
                        self.entities.push(Entity {
                            x,
                            y,
                            r: self.rng.gen(),
                            g: self.rng.gen(),
                            b: self.rng.gen(),
                            a: 1.0,
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
