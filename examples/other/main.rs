use std::f32::consts::{PI, TAU};
use std::iter::repeat;
use std::time::Duration;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use neo_granseal::{
    prelude::*,
    util::*,
    mesh::{
        FillStyle::*,
        *,
    },
};
use neo_granseal::events::{Key, KeyState, MouseButton};


pub const TILE_SCALE: u32 = 64;
pub const WIDTH: u32 = TILE_SCALE * 14;
pub const HEIGHT: u32 = TILE_SCALE * 12;


fn main() {
    start(Game::new(), GransealGameConfig::new()
              .vsync(false)
              .size(WIDTH as i32, HEIGHT as i32)
              .clear_color(Color::CORNFLOWER_BLUE)
    )
}

pub struct Level {
    tiles: Vec<Vec<u8>>,
    tile_scale: u32,
    hit_boxes: Vec<Rectangle>,
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
        };
        s.generate_hitboxs();
        s
    }
    pub fn generate_hitboxs(&mut self) {
        self.hit_boxes.clear();
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
                mb.set_cursor(Point::new(tx,ty));
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
        mb.set_cursor(Point::new(500,1000));
        mb.set_thickness(1.0);
        mb.draw_text(&rusttype::Font::try_from_bytes(include_bytes!("../../DroidSerif-Regular.ttf")).unwrap(),"Where in the world is Carmen Sandiego? I don't know, do you know? Why won't you tell me. My goodness. The Quick Brown Fox Jumped Over The Lazy Dog.",50.0);
        mb.build()
    }
}



struct Player {
    pos: Point,
    vel: Point,
    size: Point,
    mesh: Mesh,
}
impl Player {
    pub fn new(pos: Point, size: Point) -> Self {
        Self {
            pos,
            vel: Point::ZERO,
            size,
            mesh: {
                let mut mb = MeshBuilder::new();
                mb.set_cursor(Point::ZERO);
                mb.set_style(Corners(Color::random(),Color::random(),Color::random(),Color::random()));
                mb.rect(size);
                mb.set_filled(false);
                mb.set_thickness(4.0);
                mb.set_style(Solid(Color::BLACK));
                mb.set_cursor(Point::ZERO);
                mb.rect(size);
                mb.build()
            },
        }
    }
    pub fn hit_box(&self) -> Rectangle {
        Rectangle::new(self.pos.x,self.pos.y,self.size.x-0.1,self.size.y-0.1)
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    pub fn update(&mut self,level: &mut Level, delta: Duration) {
        let d = delta.as_secs_f32();

        self.pos.x += self.vel.x * d;
        level.hit_boxes.iter().for_each(|h|{
           if let Some((tl,_)) = h.overlapping_box(&self.hit_box()) {
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
            if let Some((tl,_)) = h.overlapping_box(&self.hit_box()) {
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
    level: Level,
    player: Player,
    cam: Camera,
    ray_origin: Point,
    debug: Vec<MBShapes>,
    walls: Vec<LineSegment>,
    font: rusttype::Font<'static>,
    rng: XorShiftRng,
    ray: Ray,
}
impl Game {
    pub fn new() -> Self {
        Self {
            level: Level::new(),
            player: Player::new(Point::new(128,HEIGHT - 128),Point::new(TILE_SCALE as f32 * 1.5,TILE_SCALE as f32 * 2.0)),
            cam: Camera::new(Point::new(WIDTH,HEIGHT)),
            ray_origin: Point::new(512,512),
            debug: vec![],
            walls: vec![],
            font: rusttype::Font::try_from_bytes(include_bytes!("../../DroidSerif-Regular.ttf")).unwrap(),
            rng: XorShiftRng::from_rng(rand::thread_rng()).unwrap(),
            ray: Ray::new(Point::ZERO, Point::new(1,1)),
        }
    }
}

impl NeoGransealEventHandler for Game {
    fn event(&mut self, core: &mut NGCore, e: Event) {
        let timer = core.timer.elapsed().as_secs_f32();
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
                let mp = core.state.mouse.pos + self.cam.get_offset();
                let mut g = ShapeGfx::new(core);
                g.set_offset(-self.cam.get_offset());
                g.draw_buffered_mesh(0,Point::ZERO);
                g.draw_mesh(&self.player.mesh(),self.player.pos);

                let mut mb = MeshBuilder::new();

                self.debug.iter().for_each(|s| {
                   mb.shape(*s);
                });




                let mut gs: Vec<rusttype::Glyph> = vec![];
                for c in String::from("Hello World.").chars() {
                    gs.push(self.font.glyph(c));
                }

                g.draw_mesh(&mb.build(),Point::new(300,900));
                g.draw_buffered_mesh(1,Point::new(300,1000));
                g.draw_buffered_mesh(2,Point::new(400,800));

                g.finish();
            }
            Event::Update(d) => {
                let time = core.timer.elapsed().as_secs_f32();
                self.debug.clear();
                let mp = core.state.mouse.pos + self.cam.get_offset();

                let mut state = MBState::new();


                let gravity = 1600.0;
                let player_speed = 800.0;

                let delta = d.as_secs_f32();
                core.set_title(format!("Neo-Granseal: {}",core.state.fps));
                //if core.key_held(Key::W) { self.player.vel.y -= delta * player_speed }
                if core.key_held(Key::A) { self.player.vel.x -= delta * player_speed }
                if core.key_held(Key::S) { self.player.vel.y += delta * player_speed }
                if core.key_held(Key::D) { self.player.vel.x += delta * player_speed }
                self.player.vel.y += gravity * delta;
                self.player.update(&mut self.level,d);
                self.player.vel *= 1.0 - delta;
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
                let mut mb = MeshBuilder::new();
                mb.set_style(FillStyle::Solid(Color::FIRE_BRICK));
                mb.draw_text(&self.font,"Hello Text.",22.0);
                mb.set_filled(false);
                mb.set_style(FillStyle::Solid(Color::ORANGE));
                mb.set_cursor(Point::new(900,900));
                mb.draw_text(&self.font,"Break a leg.",122.0);
                let text = mb.build();
                core.buffer_object(0,text);

                let mut pb = PathBuilder::new();
                pb.move_to(Point::new(50,0));
                let radius = 50.0;
                let count = 100;
                (0..=count).for_each(|i|{
                   let a = (TAU / count as f32) * i as f32;
                    pb.line_to(Point::new(radius * a.cos(),radius * a.sin()));
                });
                pb.close_path();

                let path = pb.build();
                let polygon = path_to_polygon(&path, 4.0);
                let mut mb = MeshBuilder::new();
                for (start,end) in polygon.edges {
                    mb.line(polygon.points[start],polygon.points[end]);
                }
                let mesh = mb.build();
                core.buffer_object(0,mesh);
            }
            _ => {}
        }
    }
}

