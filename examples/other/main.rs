use std::{
    collections::VecDeque,
    f32::consts::TAU,
    ops::Rem,
    time::Instant
};
use rand::{Rng, SeedableRng};
use neo_granseal::{
    core::NGCommand,
    start,
    GransealGameConfig,
    NeoGransealEventHandler,
    core::NGCore,
    events::Event,
    shape_pipeline::ShapeGfx,
    events::{Key, KeyState},
    mesh::{FillStyle},
    util::{Color, Point},
    mesh::*,
    mesh::FillStyle::{FadeDown, Solid},
    shape_pipeline::BufferedObjectID
};

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 600;

fn main() {
    start(Game {
        rng: rand_xorshift::XorShiftRng::from_rng(rand::thread_rng()).expect("Get Rng"),
        timer: Instant::now(),
    },
          GransealGameConfig::new()
              .vsync(false)
              .size(WIDTH,HEIGHT)
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
    rng: rand_xorshift::XorShiftRng,
    timer: Instant,
}
fn grid(size: Point) -> Mesh {
    let mut mb = MeshBuilder::new();
    mb.set_filled(false);
    mb.set_style(Solid(Color::WHITE));
    for x in (0..WIDTH).step_by(size.x as usize){
        for y in (0..HEIGHT).step_by(size.y as usize) {
            mb.rect(Point::new(x,y));
        }
    }
    mb.build()
}
impl NeoGransealEventHandler for Game {
    fn event(&mut self, core: &mut NGCore, e: Event) {
        match e {
            Event::Draw => {
                use FillStyle::*;
                let time = core.timer.elapsed().as_secs_f32();
                let mut g = ShapeGfx::new(core);
                g.set_rotation(0.0);
                g.set_fill_style(FillStyle::Solid(Color::GOLD));
                g.rect(Point::new(200,200),Point::new(150,200));
                g.set_rotation_origin(Point::new(WIDTH / 2, HEIGHT / 2));

                (0..10).for_each(|i|{
                    g.set_rotation(i as f32 + time / 4.0);
                    g.draw_buffered_mesh(0,Point::new(0,0));
                    g.set_rotation(i as f32 + -time / 4.0);
                    g.draw_buffered_mesh(0,Point::new(0,0));
                });

                g.set_rotation(0.0);
                g.set_fill_style(FillStyle::Solid(Color::MAGENTA));
                g.rect(Point::new(400,400),Point::new(150,20));

                g.finish();
            }
            Event::Update(d) => {
                core.cmd(NGCommand::SetTitle(format!("Other Stuff (experiments): {}",core.state.fps)));
            }
            Event::Load => {
                let _ = core.buffer_object(grid(Point::new(32,32)));
            }
            _ => {}
        }
    }
}