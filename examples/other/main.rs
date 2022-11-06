use std::time::Instant;
use rand::SeedableRng;
use neo_granseal::{core::NGCommand, start, GransealGameConfig, core::NGCore, events::Event, shape_pipeline::ShapeGfx, events::{Key, KeyState}, mesh::{FillStyle}, util::{Color, Point}, mesh::*, mesh::FillStyle::{FadeDown, Solid}, shape_pipeline::BufferedObjectID, MSAA, NeoGransealEventHandler};

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
    mb.set_cursor(1.0,1.0);
    mb.set_thickness(2.0);
    let size = size;
    for x in (0..WIDTH + size.x as i32).step_by(size.x as usize){
        for y in (0..HEIGHT + size.y as i32).step_by(size.y as usize) {
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

                let mut mb = MeshBuilder::new();
                mb.set_thickness(32.0);
                mb.set_rotation(time,Point::ZERO);
                mb.set_cursor(WIDTH/2,HEIGHT/2);
                mb.line(Point::new(0,0),Point::new(WIDTH,HEIGHT));
                mb.set_rotation(0.0,Point::ZERO);
                mb.line(Point::new(WIDTH,HEIGHT),Point::ZERO);
                mb.set_cursor(0,0);
                mb.rect(Point::new(44,44));
                mb.set_style(Corners(Color::RED,Color::LIME,Color::BLUE,Color::WHITE));
                // mb.quad_raw(Point::new(64, 0), Point::new(WIDTH - 64, 0), Point::new(WIDTH, HEIGHT), Point::new(0, HEIGHT));
                // mb.triangle_raw(Point::new(80, 80), Point::new(400, 400), Point::new(80, 300));
                let m = mb.build();

                let mut g = ShapeGfx::new(core);
                g.set_rotation(0.0);
                g.set_fill_style(Solid(Color::SLATE_BLUE));
                g.rect(Point::new(128,128),Point::new(256,128));
                g.set_rotation_origin(Point::new(WIDTH / 2, HEIGHT / 2));
                g.set_rotation(time);
                g.draw_buffered_mesh(2,Point::new((time * 4.0).cos() * 32.0,(time * 4.0).sin() * 32.0));
                g.draw_buffered_mesh(1,Point::new((time * 4.0).sin() * 32.0,(time * 4.0).cos() * 32.0));
                g.set_rotation(0.0);
                g.set_fill_style(Solid(Color::MAGENTA));
                g.rect(Point::new(512,256),Point::new(128,256));

                g.draw_mesh(m,Point::ZERO);

                g.finish();
            }
            Event::Update(d) => {
                core.cmd(NGCommand::SetTitle(format!("Other Stuff: {}",core.state.fps)));
            }
            Event::Load => {
                let grid_id = core.buffer_object(0,grid(Point::new(32,32)));
                let grid2_id = core.buffer_object(1,grid(Point::new(16,16)));
                let grid3_id = core.buffer_object(2,grid(Point::new(64,64)));
                println!("1: {}, 2: {}, 3: {}",grid_id,grid2_id,grid3_id);
            }
            _ => {}
        }
    }
}