use std::collections::VecDeque;
use std::f32::consts::TAU;
use std::ops::Rem;
use std::time::Instant;
use rand::{Rng, SeedableRng};
use neo_granseal::{start, GransealGameConfig, NeoGransealEventHandler, core::NGCore, events::Event, shape_pipeline::ShapeGfx};
use neo_granseal::core::NGCommand;
use neo_granseal::events::{Key, KeyState};
use neo_granseal::mesh::{FillStyle};
use neo_granseal::util::{Color, Point};
use neo_granseal::mesh::*;
use neo_granseal::mesh::FillStyle::{FadeDown, Solid};
use neo_granseal::shape_pipeline::BufferedObjectID;

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 600;

fn main() {
    start(Game {
        title: "Other example".to_string(),
        entities: vec![],
        rng: rand_xorshift::XorShiftRng::from_rng(rand::thread_rng()).expect("Get Rng"),
        points: vec![],
        queue: VecDeque::new(),
        timer: Instant::now(),
        center: Point::new(0.0,0.0),
        meshes: vec![],
        buffered_objects: vec![],
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
    title: String,
    rng: rand_xorshift::XorShiftRng,
    entities: Vec<Entity>,
    points: Vec<Point>,
    queue: VecDeque<Point>,
    timer: Instant,
    center: Point,
    meshes: Vec<Mesh>,
    buffered_objects: Vec<BufferedObjectID>,
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
                let time = core.timer.elapsed().as_secs_f32();
                let mut g = ShapeGfx::new(core);
                g.set_rotation(0.0);
                g.set_fill_style(FillStyle::Solid(Color::GOLD));
                g.rect(Point::new(200,200),Point::new(150,200));
                g.set_rotation_origin(Point::new(WIDTH / 2, HEIGHT / 2));

                (0..10).for_each(|i|{
                    g.set_rotation(i as f32 + time / 4.0);
                    g.draw_buffered_mesh(*self.buffered_objects.first().unwrap(),Point::new(0,0));
                    g.set_rotation(i as f32 + -time / 4.0);
                    g.draw_buffered_mesh(*self.buffered_objects.first().unwrap(),Point::new(0,0));
                });

                g.set_rotation(0.0);
                g.set_fill_style(FillStyle::Solid(Color::MAGENTA));
                g.rect(Point::new(400,400),Point::new(150,20));

                g.finish();
            }
            Event::Update(d) => {

            }
            Event::Load => {
                let mut mesh_builder = MeshBuilder::new();
                mesh_builder.set_filled(false);
                let size = 8;
                let o_size = Point::new(size,size);
                //mesh_builder.set_rotation(self.timer.elapsed().as_secs_f32().rem(TAU),-Point::new(size,size * 2) / 4.0);
                //mesh_builder.set_style(FadeDown(Color::RED,Color::LIME));
                //mesh_builder.set_filled(false);
                //mesh_builder.set_thickness(16.0);
                let mut count = 0;
                for x in (0..WIDTH).step_by(size) {
                    for y in (0..HEIGHT).step_by(size) {
                        count += 1;
                        mesh_builder.set_cursor(x,y);
                        mesh_builder.set_style(Solid(Color::rgb(self.rng.gen(),self.rng.gen(),self.rng.gen())));
                        mesh_builder.rect(o_size);
                    }
                }
                println!("Verticies: {}",count * 4);
                //self.meshes.push(mesh_builder.build());

                let oval_wall = core.buffer_object(mesh_builder.build());
                self.buffered_objects.push(oval_wall);
            }
            _ => {}
        }
    }
}