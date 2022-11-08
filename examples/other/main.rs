use std::time::Duration;
use num_traits::AsPrimitive;
use rand::SeedableRng;
use neo_granseal::{core::NGCommand, start, GransealGameConfig, core::NGCore, events::Event, shape_pipeline::ShapeGfx, events::{Key, KeyState}, mesh::{FillStyle}, util::{Color, Point}, mesh::*, mesh::FillStyle::{FadeDown, Solid}, shape_pipeline::BufferedObjectID, MSAA, NeoGransealEventHandler};
use neo_granseal::events::MouseButton;
use neo_granseal::mesh::FillStyle::{Corners, Radial};
use neo_granseal::util::{Camera, Rectangle};

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
    hit_boxes: Vec<Rectangle>,
    debug_meshes: Vec<Mesh>,
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
            debug_meshes: vec![],
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
    pub fn intersects(&self, hbox: &Rectangle) -> bool {
        for hb in self.hit_boxes.iter() {
            if hb.intersects(hbox) {
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
        Rectangle::new(self.pos.x,self.pos.y,self.size.x-1.0,self.size.y-1.0)
    }
    pub fn mesh(&self) -> Mesh {
        let mut mb = MeshBuilder::new();
        mb.set_cursor(0,0);
        mb.set_style(Corners(Color::RED,Color::LIME,Color::BLUE,Color::BLACK));
        mb.rect(self.size);
        mb.set_filled(false);
        mb.set_style(Solid(Color::DIM_GRAY));
        mb.set_cursor(0,0);
        mb.rect(self.size);
        mb.build()
    }
    pub fn update(&mut self,level: &mut Level, delta: Duration) {
        let d = delta.as_secs_f32();
        let mut mb = MeshBuilder::new();
        //mb.set_cursor(self.pos.x, self.pos.y);
        mb.set_style(Solid(Color::HOT_PINK));

        self.pos.x += self.vel.x * d;
        self.pos.y += self.vel.y * d;

        let phb = self.hit_box();
        let mut hits: Vec<(Point,Point)> = vec![];
        level.hit_boxes.iter().for_each(|h| {
            if let Some((tl,br)) = phb.overlapping_box(h) {
                hits.push((tl,br));
                mb.set_style(Solid(Color::HOT_PINK));
                mb.line(tl,br);
                if br.x - tl.x < br.y - tl.y {
                    if self.vel.x > 0.0 {
                        self.pos.x -= br.x - tl.x;
                        self.vel.x = 0.0;
                    }else{
                        self.pos.x += br.x - tl.x;
                        self.vel.x = 0.0;
                    }
                }else{
                    if self.vel.y > 0.0 {
                        self.pos.y -= br.y - tl.y;
                        self.vel.y = 0.0;
                    }else{
                        self.pos.y += br.y - tl.y;
                        self.vel.y = 0.0;
                    }
                }


                mb.set_style(Solid(Color::LIME));

            }
        });


        level.debug_meshes.push(mb.build());

    }
}
struct Game {
    rng: rand_xorshift::XorShiftRng,
    level: Level,
    player: Player,
    cam: Camera,
    debug: Vec<Mesh>,
}
impl Game {
    pub fn new() -> Self {
        Self {
            rng: rand_xorshift::XorShiftRng::from_rng(rand::thread_rng()).expect("get Rng"),
            level: Level::new(),
            player: Player::new(Point::new(128,HEIGHT - 128),Point::new(TILE_SCALE,TILE_SCALE)),
            cam: Camera::new(Point::new(WIDTH,HEIGHT)),
            debug: vec![],
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
                    let mp = core.state.mouse.pos + self.cam.get_offset();
                    let halfs = 32.0;
                    let test_box = Rectangle::new(mp.x - halfs,mp.y - halfs,halfs * 2.0,halfs * 2.0);
                    let intersect = test_box.overlapping_box(&self.player.hit_box());
                    let mut mb = MeshBuilder::new();
                    mb.set_cursor(test_box.top_left.x,test_box.top_left.y);
                    mb.rect(Point::new(test_box.bottom_right.x - test_box.top_left.x,test_box.bottom_right.y - test_box.top_left.y));
                    mb.set_style(Solid(Color::LIME));
                    if let Some((tl,br)) = intersect {
                        mb.set_style(Solid(Color::RED));
                        mb.set_cursor(tl.x,tl.y);
                        mb.rect(Point::new(br.x,br.y));
                    }
                    self.debug.clear();
                    self.debug.push(mb.build());
                }
            }
            Event::Draw => {
                let mut g = ShapeGfx::new(core);
                g.set_position(-self.cam.get_offset());
                g.draw_buffered_mesh(0,Point::ZERO);
                g.draw_mesh(self.player.mesh(),self.player.pos);
                self.debug.iter_mut().for_each(|m| {
                    g.set_fill_style(Solid(Color::RED));
                    g.draw_mesh(m.clone(),Point::ZERO);
                });
                g.finish();
            }
            Event::Update(d) => {
                let gravity = 000.0;
                let player_speed = 400.0;
                let delta = d.as_secs_f32();
                core.cmd(NGCommand::SetTitle(format!("Gario, Neo-Granseal: {}",core.state.fps)));
                if core.key_held(Key::W) { self.player.vel.y -= delta * player_speed }
                if core.key_held(Key::A) { self.player.vel.x -= delta * player_speed }
                if core.key_held(Key::S) { self.player.vel.y += delta * player_speed }
                if core.key_held(Key::D) { self.player.vel.x += delta * player_speed }
                self.player.vel.y += gravity * delta;
                self.player.update(&mut self.level,d);
                self.cam.target(self.player.pos);
                self.debug.clear();
                if !self.level.debug_meshes.is_empty() {
                    self.debug.push(self.level.debug_meshes.pop().unwrap());

                }

            }
            Event::Load => {
                core.buffer_object(0,self.level.level_mesh());
                self.cam.set_bounds(Point::new(0,0),Point::new(self.level.width() * TILE_SCALE,self.level.height()*TILE_SCALE));
            }
            _ => {}
        }
    }
}

