use neo_granseal::math::Vec2;
use neo_granseal::mesh::{FillStyleShorthand, MeshBuilder};
use neo_granseal::shape_pipeline::ShapeGfx;
use neo_granseal::util::Color;
use neo_granseal::{
    core::NGCore, events::Event, start, GransealGameConfig, NeoGransealEventHandler,
};
use rand::{Rng, SeedableRng};

fn main() {
    start(
        Game::new(),
        GransealGameConfig::new()
            .clear_color(Color::BLACK)
            .vsync(false)
            .size(128 * 5, 128 * 5),
    );
}
struct Entity {
    pos: Vec2,
    color: Color,
    rot: f32,
}
struct Game {
    rng: rand_xorshift::XorShiftRng,
    entities: Vec<Entity>,
    size: Vec2,
    timer: std::time::Instant,
    toggle: bool,
}
impl Game {
    fn new() -> Self {
        Self {
            rng: rand_xorshift::XorShiftRng::from_rng(rand::thread_rng()).expect("Getting Rng."),
            entities: vec![],
            size: Vec2::new(16.0, 16.0),
            timer: std::time::Instant::now(),
            toggle: true,
        }
    }
}

impl NeoGransealEventHandler for Game {
    fn event(&mut self, core: &mut NGCore, e: Event) {
        match e {
            Event::Draw => {
                let mut mb = MeshBuilder::new();
                self.entities.iter().enumerate().for_each(|(i, e)| {
                    mb.solid(e.color);
                    mb.set_cursor(e.pos);
                    mb.set_rotation(
                        core.timer.elapsed().as_secs_f32() * (i as f32) / 100.0,
                        self.size / 2.0,
                    );
                    mb.rect(self.size);
                });
                let mut g = ShapeGfx::new(core);
                g.draw_mesh(&mb.build(), Vec2::ZERO);
                g.finish();
            }
            Event::Update(d) => {
                core.set_title(format!(
                    "Drawing a lot of boxes dynamically: {}",
                    core.state.fps
                ));
                self.entities
                    .iter_mut()
                    .for_each(|e| e.rot += d.as_secs_f32() * e.color.r);
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
                for x in (0..core.config.width as usize).step_by(self.size.x.floor() as usize) {
                    for y in (0..core.config.height as usize).step_by(self.size.y.floor() as usize)
                    {
                        let pos = Vec2::new(x as f32, y as f32);
                        let color = Color::rgb(self.rng.gen(), self.rng.gen(), self.rng.gen());
                        self.entities.push(Entity {
                            pos,
                            color,
                            rot: 0.0,
                        })
                    }
                }
                println!("Boxes: {}", self.entities.len());
            }
            _ => {}
        }
    }
}
