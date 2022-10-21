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
            .vsync(VSyncMode::FastVSync)
            .size(128 * 5, 128 * 5),
    );
}
struct Entity {
    x: f32,
    y: f32,
}
struct Game {
    rng: rand_xorshift::XorShiftRng,
    gfx: SSRGraphics,
    entities: Vec<Entity>
}
impl Game {
    fn new() -> Self {
        Self {
            rng: rand_xorshift::XorShiftRng::from_rng(rand::thread_rng()).expect("Getting Rng."),
            gfx: SSRGraphics::new(),
            entities: vec![],
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
                    self.gfx.fill_rgba(self.rng.gen(),self.rng.gen(),self.rng.gen(),1.0);
                    self.gfx.fill_rect(e.x,e.y,16.0,16.0);
                }
                core.cmd(NGCommand::Render(0, Box::new(self.gfx.data.to_owned())));
            }
            Event::Update(_) => {}
            Event::Load => {
                let step: usize = 16;
                let half = step / 2;
                for x in (half..core.config.width as usize).step_by(step) {
                    for y in (half..core.config.height as usize).step_by(step) {
                        let x = x as f32;
                        let y = y as f32;
                        self.entities.push(Entity {
                            x,
                            y,
                        })
                    }
                }
            }
            Event::Resized(_, _) => {}
            Event::Fps(_) => {}
        }
    }
}
