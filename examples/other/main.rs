use std::{time::Duration, f32::consts::PI};
use num_traits::AsPrimitive;
use rand::SeedableRng;
use neo_granseal::{core::NGCommand, start, GransealGameConfig, core::NGCore, events::Event, shape_pipeline::ShapeGfx, events::{Key, KeyState}, mesh::{FillStyle}, util::{Color, Point}, mesh::*, mesh::FillStyle::{FadeDown, Solid}, shape_pipeline::BufferedObjectID, MSAA, NeoGransealEventHandler};
use neo_granseal::events::MouseButton;
use neo_granseal::mesh::FillStyle::{Corners, Radial};
use neo_granseal::util::{Camera, Rectangle};

pub const TILE_SCALE: u32 = 39;
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
    hit_boxes: Vec<Rectangle>,
    walls: Vec<LineSegment>,
}
impl Level {
    pub fn new() -> Self {
        let mut s = Self  {
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
            tile_scale: TILE_SCALE as u32,
            hit_boxes: vec![],
            walls: vec![],
        };
        s.generate_hitboxs();
        s
    }
    pub fn generate_hitboxs(&mut self) {
        let width = self.width();
        let height = self.height();
        for x in 0..width {
            for y in 0..height {
                let tx = x * self.tile_scale;
                let ty = y * self.tile_scale;
                let tile_type = self.get_tile(x, y);
                match tile_type {
                    b'g' => {
                        self.hit_boxes.push(
                            Rectangle::new(tx,ty,self.tile_scale,self.tile_scale)
                        );
                    }
                    b'x' => {
                        self.hit_boxes.push(
                            Rectangle::new(tx,ty,self.tile_scale,self.tile_scale)
                        );
                    }
                    _ => {}
                }
            }
        }
    }
    pub fn get_tile(&self, x: u32, y: u32) -> u8 {
        let lines = self.tiles.as_slice();
        if y as usize >= lines.len() {return b'x'}
        if x as usize >= lines[0].len() {return b'x'}
        lines[y as usize][x as usize]
    }
    pub fn is_blocking(&self, p: Point) -> bool {
        let x = (p.x / TILE_SCALE as f32) as u32;
        let y = (p.y / TILE_SCALE as f32) as u32;
        match self.get_tile(x,y) {
            b'g' => {true}
            b'x' => {true}
            _ => {false}
        }
    }
    pub fn intersects(&self, hbox: &Rectangle) -> bool {
        for hb in self.hit_boxes.iter() {
            if hb.intersects_rect(hbox) {
                return true;
            }
        }
        false
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

pub struct LineSegment {
    begin: Point,
    end: Point,
}
impl LineSegment {
    pub fn new(begin: Point, end: Point) -> Self {
        Self {
            begin,
            end,
        }
    }
    pub fn normal(&self) -> Point {
        let d = (self.end - self.begin);
        let a = d.angle() - PI/2.0;
        Point::new(a.cos(),a.sin())
    }
    pub fn length(&self) -> f32 {
        let d = (self.end - self.begin);
        d.len()
    }
    pub fn axis(&self) -> Point {
        let d = (self.end - self.begin);
        let a = d.angle();
        Point::new(a.cos(),a.sin())
    }
    pub fn debug(&self, mb: &mut MeshBuilder) {
        mb.push();
        mb.set_resolution(1.0);
        mb.set_cursor_p(Point::ZERO);
        mb.set_thickness(1.0);
        mb.set_style(Radial(Color::DEEP_PINK,Color::HOT_PINK));
        mb.set_cursor_p(self.begin);
        mb.oval(Point::new(4,4));
        mb.set_cursor_p(self.end);
        mb.oval(Point::new(4,4));
        mb.set_cursor_p(Point::ZERO);
        mb.set_style(Solid(Color::INDIGO));
        mb.line(self.begin,self.end);
        mb.set_style(Solid(Color::BLUE));
        mb.set_thickness(1.0);
        mb.line(self.begin + self.axis() * (self.length()/2.0),self.begin+ self.axis() * (self.length()/2.0) + self.normal() * 32.0);
        mb.pop();
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
        Rectangle::new(self.pos.x,self.pos.y,self.size.x-0.1,self.size.y-0.1)
    }
    pub fn mesh(&self) -> Mesh {
        let mut mb = MeshBuilder::new();
        mb.set_cursor(0,0);
        mb.set_style(Corners(Color::RED,Color::LIME,Color::BLUE,Color::BLACK));
        mb.rect(self.size);
        mb.set_filled(false);
        mb.set_thickness(4.0);
        mb.set_style(Solid(Color::BLACK));
        mb.set_cursor(0,0);
        mb.rect(self.size);
        mb.build()
    }
    pub fn update(&mut self,level: &mut Level, delta: Duration) {
        let d = delta.as_secs_f32();

        self.pos.x += self.vel.x * d;
        level.hit_boxes.iter().for_each(|h|{
           if let Some((tl,br)) = h.overlapping_box(&self.hit_box()) {
               if self.pos.x + self.size.x/2.0 > tl.x  {
                   self.pos.x -= self.vel.x * d;
                   self.vel.x = 0.0;
               } else {
                   self.pos.x -= self.vel.x * d;
                   self.vel.x = 0.0;
               }
           }
        });

        self.pos.y += self.vel.y * d;
        level.hit_boxes.iter().for_each(|h|{
            if let Some((tl,br)) = h.overlapping_box(&self.hit_box()) {
                if self.pos.y + self.size.y/2.0 > tl.y  {
                    self.pos.y -= self.vel.y * d;
                    self.vel.y = 0.0;
                } else {
                    self.pos.y -= self.vel.y * d;
                    self.vel.y = 0.0;
                }
            }
        });

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
            player: Player::new(Point::new(128,HEIGHT - 128),Point::new(TILE_SCALE,TILE_SCALE)),
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
                        self.player.vel.y -= 900.0;
                    }
                }
            }
            Event::MousePressed {button,state} => {
                if button == MouseButton::Left && state == KeyState::Pressed {
                    let mp = core.state.mouse.pos + self.cam.get_offset();
                }
            }
            Event::Draw => {
                let mut g = ShapeGfx::new(core);
                g.set_position(-self.cam.get_offset());
                g.draw_buffered_mesh(0,Point::ZERO);
                g.draw_mesh(self.player.mesh(),self.player.pos);

                let mut mb = MeshBuilder::new();

                self.level.hit_boxes.iter().for_each(|h|{
                   if h.test {
                       mb.set_cursor_p(h.top_left);
                       mb.set_filled(false);
                       mb.rect(h.bottom_right - h.top_left);
                   }
                });
                g.draw_mesh(mb.build(), Point::ZERO);
                g.finish();
            }
            Event::Update(d) => {
                let gravity = 1600.0;
                let player_speed = 800.0;
                let delta = d.as_secs_f32();
                core.cmd(NGCommand::SetTitle(format!("Neo-Granseal: {}",core.state.fps)));
                //if core.key_held(Key::W) { self.player.vel.y -= delta * player_speed }
                if core.key_held(Key::A) { self.player.vel.x -= delta * player_speed }
                if core.key_held(Key::S) { self.player.vel.y += delta * player_speed }
                if core.key_held(Key::D) { self.player.vel.x += delta * player_speed }
                self.player.vel.y += gravity * delta;
                self.player.update(&mut self.level,d);
                self.player.vel *= (1.0 - d.as_secs_f32() * 2.0);
                self.cam.target(self.player.pos);
                self.level.hit_boxes.iter_mut().for_each(|h|{
                   if h.contains_point(&(core.state.mouse.pos + self.cam.get_offset())) {
                       h.test = true;
                   }  else {h.test = false;}
                });
            }
            Event::Load => {
                core.buffer_object(0,self.level.level_mesh());
                self.cam.set_bounds(Point::new(0,0),Point::new(self.level.width() * TILE_SCALE,self.level.height()*TILE_SCALE));
            }
            _ => {}
        }
    }
}

