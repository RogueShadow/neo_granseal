use std::time::{Duration, Instant};
use num_traits::AsPrimitive;
use rand::SeedableRng;
use neo_granseal::{core::NGCommand, start, GransealGameConfig, core::NGCore, events::Event, shape_pipeline::ShapeGfx, events::{Key, KeyState}, mesh::{FillStyle}, util::{Color, Point}, mesh::*, mesh::FillStyle::{FadeDown, Solid}, shape_pipeline::BufferedObjectID, MSAA, NeoGransealEventHandler};
use neo_granseal::events::MouseButton;
use neo_granseal::mesh::FillStyle::Corners;
use neo_granseal::util::Camera;

pub const TILE_SCALE: u32 = 64;
pub const WIDTH: u32 = 12 * TILE_SCALE;
pub const HEIGHT: u32 = 10 * TILE_SCALE;

fn main() {
    start(Game::new(),GransealGameConfig::new()
              .vsync(false)
              .size(WIDTH as i32, HEIGHT as i32)
              .clear_color(Color::CORNFLOWER_BLUE)
    )
}

pub struct Level {
    tiles: Vec<Vec<u8>>,
    tile_scale: u32,
}
impl Level {
    pub fn new() -> Self {
        Self  {
            tiles: vec![
                b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_vec(),
                b"x.....x..................................................x".to_vec(),
                b"x.....x..................................................x".to_vec(),
                b"x.....x..................................................x".to_vec(),
                b"x.....x..................................................x".to_vec(),
                b"x.....x..................................................x".to_vec(),
                b"x.....x..................................................x".to_vec(),
                b"x.....x..................................................x".to_vec(),
                b"x.....x..................................................x".to_vec(),
                b"x.....x..................................................x".to_vec(),
                b"x.....x..................................................x".to_vec(),
                b"x...........................x............................x".to_vec(),
                b"x..........................x.............................x".to_vec(),
                b"x.........................x..............................x".to_vec(),
                b"x........................x...............................x".to_vec(),
                b"x.......................x................................x".to_vec(),
                b"x......................x.................................x".to_vec(),
                b"x..x..x..x.x..........x..................................x".to_vec(),
                b"x....x.....x.........x...................................x".to_vec(),
                b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_vec()
            ],
            tile_scale: TILE_SCALE as u32
        }
    }
    pub fn get_tile(&self, x: u32, y: u32) -> u8 {
        let lines = self.tiles.as_slice();
       // if self.tiles.len() > y as usize {return b' '}
        //if self.tiles[0].len() > x as usize {return b' '}
        lines[y as usize][x as usize]
    }
    pub fn is_blocking(&self, x: impl AsPrimitive<f32>, y: impl AsPrimitive<f32>) -> bool {
        let x = (x.as_() / TILE_SCALE as f32) as u32;
        let y = (y.as_() / TILE_SCALE as f32) as u32;
        match self.get_tile(x,y) {
            b'x' => {true}
            _ => {false}
        }
    }
    pub fn width(&self) -> u32 {
        self.tiles[0].len() as u32
    }
    pub fn height(&self) -> u32 {
        self.tiles.len() as u32
    }
    pub fn level_mesh(&self) -> Mesh {
        let width = self.width();
        let height = self.height();
        let tile_size = Point::new(self.tile_scale,self.tile_scale);
        let mut mb = MeshBuilder::new();
        for x in 0..width {
            for y in 0..height {
                let tx = x * self.tile_scale;
                let ty = y * self.tile_scale;
                let tile_type = self.get_tile(x,y);
                mb.set_cursor(tx,ty);
                mb.push();
                match tile_type {
                    b'x' => {
                        mb.set_style(FadeDown(Color::DIM_GRAY,Color::FIRE_BRICK));
                        mb.rect(tile_size);
                        mb.set_style(Solid(Color::BLACK));
                        mb.set_filled(false);
                        mb.rect(tile_size);
                    }
                    _ => {}
                }
                mb.pop();
            }
        }
        mb.build()
    }
}
struct Player {
    pos: Point,
    vel: Point,
    size: Point,
}
impl Player {
    pub fn mesh(&self) -> Mesh {
        let mut mb = MeshBuilder::new();
        mb.set_cursor(0,0);
        mb.set_style(Corners(Color::RED,Color::LIME,Color::BLUE,Color::BLACK));
        mb.rect(Point::new(TILE_SCALE, TILE_SCALE));
        mb.set_style(Solid(Color::INDIAN_RED));
        mb.set_cursor(TILE_SCALE/4,TILE_SCALE/4);
        mb.oval(Point::new(TILE_SCALE/2,TILE_SCALE/8));
        mb.build()
    }
    pub fn update(&mut self,level: &Level, delta: Duration) {
        let d = delta.as_secs_f32();

        let nx = self.pos.x + self.vel.x * d;
        let ny = self.pos.y + self.vel.y * d;

        if !level.is_blocking(nx,self.pos.y) && !level.is_blocking(nx + self.size.x,self.pos.y){
            self.pos.x = nx;
        }else{
            self.vel.x = 0.0;
        }

        if !level.is_blocking(self.pos.x,ny) && !level.is_blocking(self.pos.x,ny + self.size.y){
            self.pos.y = ny;
        }else{
            self.vel.y = 0.0;
        }

    }
}
struct Game {
    rng: rand_xorshift::XorShiftRng,
    level: Level,
    player: Player,
    cam: Camera,
}
impl Game {
    pub fn new() -> Self {
        Self {
            rng: rand_xorshift::XorShiftRng::from_rng(rand::thread_rng()).expect("get Rng"),
            level: Level::new(),
            player: Player  {pos: Point::new(64,64), vel: Point::ZERO, size: Point::new(TILE_SCALE,TILE_SCALE)},
            cam: Camera::new(Point::new(WIDTH,HEIGHT))
        }
    }
}

impl NeoGransealEventHandler for Game {
    fn event(&mut self, core: &mut NGCore, e: Event) {
        match e {
            Event::KeyEvent {key, state} => {
                if state == KeyState::Pressed {
                    if key == Key::W {
                        self.player.vel.y -= 500.0;
                    }
                }
            }
            Event::MousePressed {button,state} => {
                if button == MouseButton::Left && state == KeyState::Pressed {
                    let mp = core.state.mouse.pos + self.cam.offset;
                    if self.level.is_blocking(mp.x,mp.y) {
                        println!("Wall")
                    }else{println!("Not Wall")}
                }
            }
            Event::Draw => {
                let mut g = ShapeGfx::new(core);
                g.set_position(self.cam.get_offset());
                g.draw_buffered_mesh(0,Point::ZERO);
                g.draw_mesh(self.player.mesh(),self.player.pos);
                g.finish();
            }
            Event::Update(d) => {
                let gravity = 1000.0;
                let player_speed = 400.0;
                let delta = d.as_secs_f32();
                core.cmd(NGCommand::SetTitle(format!("Gario, Neo-Granseal: {}",core.state.fps)));
                //if core.key_held(Key::W) { self.player.vel.y -= delta * gravity*20.0 }
                if core.key_held(Key::A) { self.player.vel.x -= delta * player_speed }
                if core.key_held(Key::S) { self.player.vel.y += delta * player_speed }
                if core.key_held(Key::D) { self.player.vel.x += delta * player_speed }
                self.player.vel.y += gravity * delta;
                self.player.update(&self.level,d);


                self.cam.target(self.player.pos);

            }
            Event::Load => {
                core.buffer_object(0,self.level.level_mesh());
                self.cam.set_bounds(Point::new(0,0),Point::new(self.level.width() * TILE_SCALE,self.level.height()*TILE_SCALE));
            }
            _ => {}
        }
    }
}

