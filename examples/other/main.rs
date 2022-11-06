use std::time::{Duration, Instant};
use num_traits::AsPrimitive;
use rand::SeedableRng;
use neo_granseal::{core::NGCommand, start, GransealGameConfig, core::NGCore, events::Event, shape_pipeline::ShapeGfx, events::{Key, KeyState}, mesh::{FillStyle}, util::{Color, Point}, mesh::*, mesh::FillStyle::{FadeDown, Solid}, shape_pipeline::BufferedObjectID, MSAA, NeoGransealEventHandler};
use neo_granseal::events::MouseButton;
use neo_granseal::mesh::FillStyle::{Corners, Radial};
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
                b"gggggggggggggggggggggggggggggggggggggggggggggggggggggggggg".to_vec()
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
            b'g' => {true}
            b'x' => {true}
            _ => {false}
        }
    }
    pub fn intersects(&self, hbox: Rectangle) -> bool {

        true
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
                    b'g' => {
                        mb.set_style(FadeDown(Color::BLACK,Color::SADDLE_BROWN));
                        mb.rect(tile_size);
                        mb.set_thickness(16.0);
                        mb.set_line_style(LineStyle::Right);
                        mb.set_style(FadeDown(Color::BLACK,Color::GREEN));
                        mb.set_filled(true);
                        mb.line(Point::ZERO,Point::new(self.tile_scale,0));
                    }
                    b'x' => {
                        mb.set_style(FadeDown(Color::DIM_GRAY,Color::BLACK));
                        mb.rect(tile_size);
                        mb.set_filled(false);
                        mb.set_thickness(8.0);
                        mb.set_style(Radial(Color::BLACK,Color::TRANSPARENT));
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
#[derive(Copy, Clone,PartialEq,Debug)]
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
impl Rectangle {
    pub fn new(x: impl AsPrimitive<f32>,y: impl AsPrimitive<f32>,w: impl AsPrimitive<f32>,h: impl AsPrimitive<f32>) -> Self {
        Self {
            top_left: Point::new(x,y),
            bottom_right: Point::new(x.as_()+w.as_(),y.as_()+h.as_()),
        }
    }
    pub fn intersects(&self, other: Self) -> bool {
        if  self.top_left.x > other.bottom_right.x ||
            self.bottom_right.x < other.top_left.x ||
            self.top_left.y > other.bottom_right.y ||
            self.bottom_right.y < other.top_left.y {
            false
        }else{
            true
        }
    }
}
struct Player {
    pos: Point,
    vel: Point,
    size: Point,
}
impl Player {
    pub fn new(pos: Point, size: Point) -> Self {
        Self {
            pos,
            vel: Point::ZERO,
            size,
        }
    }
    pub fn hit_box(&self) -> Rectangle {
        Rectangle::new(self.pos.x,self.pos.y,self.size.x,self.size.y)
    }
    pub fn mesh(&self) -> Mesh {
        let mut mb = MeshBuilder::new();
        mb.set_cursor(0,0);
        mb.set_style(Corners(Color::RED,Color::LIME,Color::BLUE,Color::BLACK));
        mb.rect(self.size);
        mb.set_style(Solid(Color::INDIAN_RED));
        mb.set_cursor(self.size.x/4.0,self.size.y/4.0);
        mb.oval(Point::new(self.size.x/2.0,self.size.y/8.0));
        mb.set_filled(false);
        mb.set_style(Solid(Color::GOLDENROD));
        mb.set_cursor(0,0);
        mb.rect(self.size);
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
            player: Player::new(Point::new(64,HEIGHT - 128),Point::new(TILE_SCALE,TILE_SCALE)),
            cam: Camera::new(Point::new(WIDTH,HEIGHT)),
        }
    }
}

impl NeoGransealEventHandler for Game {
    fn event(&mut self, core: &mut NGCore, e: Event) {
        match e {
            Event::KeyEvent {key, state} => {
                if state == KeyState::Pressed {
                    if key == Key::Space {
                        self.player.vel.y -= 500.0;
                    }
                }
            }
            Event::MousePressed {button,state} => {
                if button == MouseButton::Left && state == KeyState::Pressed {

                }
            }
            Event::Draw => {
                let mut g = ShapeGfx::new(core);
                g.set_position(-self.cam.get_offset());
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

