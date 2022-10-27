use std::collections::VecDeque;
use std::ops::Rem;
use std::time::Instant;
use rand::{Rng, SeedableRng};
use neo_granseal::{start, GransealGameConfig, VSyncMode, NeoGransealEventHandler, core::NGCore, events::Event, shape_pipeline::SSRGraphics};
use neo_granseal::shape_pipeline::{FillStyle, LineStyle};
use neo_granseal::util::{Color, Point};

fn main() {
    start(Game {
        entities: vec![],
        rng: rand_xorshift::XorShiftRng::from_rng(rand::thread_rng()).expect("Get Rng"),
        points: vec![],
        queue: VecDeque::new(),
        timer: Instant::now(),
    },
          GransealGameConfig::new()
              .size(128*6,128*6)
              .vsync(VSyncMode::AutoNoVsync)
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
    entities: Vec<Entity>,
    points: Vec<Point>,
    queue: VecDeque<Point>,
    timer: Instant,
}
fn grid(g: &mut SSRGraphics, screen_size: Point, grid_size: Point) {
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
            Event::KeyEvent { .. } => {}
            Event::MouseButton { .. } => {}
            Event::MouseMoved { .. } => {}
            Event::Draw => {
                use FillStyle::*;
                let width = core.config.width as f32;
                let height = core.config.height as f32;
                let time = core.timer.elapsed().as_secs_f32() / 2.0;
                let mut gfx = SSRGraphics::new(core);
                gfx.thickness = 1.0;
                gfx.color = Solid(Color::DIM_GRAY);
                grid(&mut gfx, Point::new(width,height),Point::new(32.0,32.0));
                gfx.thickness = 1.0;
                let cx = width / 2.0 - 256.0;
                let cy = height / 2.0;
                gfx.color = FadeLeft(Color::TRANSPARENT,Color::SADDLE_BROWN);
                gfx.arc(Point::new(cx,cy),64.0,90.0,180.0,4.0);
                gfx.color = FadeLeft(Color::SADDLE_BROWN,Color::TRANSPARENT);
                gfx.rect(Point::new(cx-32.0,cy-32.0),Point::new(64.0,64.0));
                gfx.color = FadeLeft(Color::TRANSPARENT,Color::SADDLE_BROWN);
                gfx.arc(Point::new(cx,cy - 64.0),64.0,180.0,270.0,4.0);
                gfx.color = FadeDown(Color::SADDLE_BROWN,Color::TRANSPARENT);
                gfx.rect(Point::new(cx+32.0,cy+32.0),Point::new(64.0,64.0));
                gfx.color = FadeLeft(Color::TRANSPARENT,Color::SADDLE_BROWN);
                gfx.arc(Point::new(cx + 64.0,cy - 64.0),64.0,270.0,360.0,4.0);
                gfx.color = FadeLeft(Color::TRANSPARENT,Color::SADDLE_BROWN);
                gfx.rect(Point::new(cx+96.0,cy-32.0),Point::new(64.0,64.0));
                gfx.color = FadeLeft(Color::TRANSPARENT,Color::SADDLE_BROWN);
                gfx.arc(Point::new(cx + 64.0,cy),64.0,0.0,90.0,4.0);
                gfx.color = FadeDown(Color::TRANSPARENT,Color::SADDLE_BROWN);
                gfx.rect(Point::new(cx+32.0,cy-96.0),Point::new(64.0,64.0));

                let cx = width / 2.0 - 32.0;
                let cy = height / 2.0;
                gfx.color = FadeLeft(Color::SADDLE_BROWN,Color::TRANSPARENT);
                //gfx.arc(Point::new(cx,cy),64.0,90.0,180.0,4.0);
                gfx.rect(Point::new(cx-32.0,cy+32.0),Point::new(64.0,64.0));

                gfx.color = FadeLeft(Color::SADDLE_BROWN,Color::TRANSPARENT);
                gfx.rect(Point::new(cx-32.0,cy-32.0),Point::new(64.0,64.0));
                gfx.color = FadeLeft(Color::TRANSPARENT,Color::SADDLE_BROWN);
                gfx.arc(Point::new(cx,cy - 64.0),64.0,180.0,270.0,4.0);
                gfx.color = FadeDown(Color::SADDLE_BROWN,Color::TRANSPARENT);
                //gfx.rect(Point::new(cx+32.0,cy+32.0),Point::new(64.0,64.0));
                gfx.color = FadeLeft(Color::TRANSPARENT,Color::SADDLE_BROWN);
                //gfx.arc(Point::new(cx + 64.0,cy - 64.0),64.0,270.0,360.0,4.0);
                gfx.color = FadeLeft(Color::TRANSPARENT,Color::SADDLE_BROWN);
                //gfx.rect(Point::new(cx+96.0,cy-32.0),Point::new(64.0,64.0));
                gfx.color = FadeLeft(Color::TRANSPARENT,Color::SADDLE_BROWN);
                //gfx.arc(Point::new(cx + 64.0,cy),64.0,0.0,90.0,4.0);
                gfx.color = FadeDown(Color::TRANSPARENT,Color::SADDLE_BROWN);
                gfx.rect(Point::new(cx+32.0,cy-96.0),Point::new(64.0,64.0));

                let cx = width / 2.0 + 256.0 - 128.0;
                let cy = height / 2.0;
                gfx.color = FadeLeft(Color::TRANSPARENT,Color::SADDLE_BROWN);
                gfx.arc(Point::new(cx,cy),64.0,90.0,180.0,4.0);
                gfx.color = FadeLeft(Color::SADDLE_BROWN,Color::TRANSPARENT);
                gfx.rect(Point::new(cx-32.0,cy-32.0),Point::new(64.0,64.0));
                gfx.color = FadeLeft(Color::TRANSPARENT,Color::SADDLE_BROWN);
                gfx.arc(Point::new(cx,cy - 64.0),64.0,180.0,270.0,4.0);
                gfx.color = FadeDown(Color::SADDLE_BROWN,Color::TRANSPARENT);
                gfx.rect(Point::new(cx+32.0,cy+32.0),Point::new(64.0,64.0));
                gfx.color = FadeLeft(Color::TRANSPARENT,Color::SADDLE_BROWN);
                gfx.arc(Point::new(cx + 64.0,cy - 64.0),64.0,270.0,360.0,4.0);
                gfx.color = FadeLeft(Color::TRANSPARENT,Color::SADDLE_BROWN);
                //gfx.rect(Point::new(cx+96.0,cy-32.0),Point::new(64.0,64.0));
                gfx.color = FadeLeft(Color::TRANSPARENT,Color::SADDLE_BROWN);
                gfx.arc(Point::new(cx + 64.0,cy),64.0,0.0,90.0,4.0);
                gfx.color = FadeDown(Color::TRANSPARENT,Color::SADDLE_BROWN);
                gfx.rect(Point::new(cx+32.0,cy-96.0),Point::new(64.0,64.0));
                for (i,p) in self.queue.iter().enumerate() {
                    //gfx.color = FadeLeft(Color::rgb(p.x/200.0,p.y/height,0.5),Color::RED);
                    //gfx.line(Point::new(i as f32 * gfx.thickness,0.0),Point::new(i as f32 * gfx.thickness,p.y))
                };
                gfx.fill = false;
                gfx.thickness = 172.0 * time.rem(3.15).sin();
                gfx.circle(Point::new(700.0,700.0),172.0,180.0);

                gfx.finish();
            }
            Event::Update(d) => {
                if self.timer.elapsed().as_secs_f32() > 0.005 {
                    self.queue.get_mut(0).unwrap().y = core.timer.elapsed().as_secs_f32().rem(3.14).sin() * 500.0 + self.rng.gen::<f32>() * 100.0;
                    self.queue.rotate_right(1);
                    self.timer = std::time::Instant::now();
                }
                self.entities.iter_mut().for_each(|e| e.update(d));
            }
            Event::Load => {
                (0..100).for_each(|i|{
                    self.points.push(
                        Point::new(
                            self.rng.gen::<f32>() * core.config.width as f32,
                            self.rng.gen::<f32>()*core.config.height as f32
                        )
                    );
                });
                (0..core.config.width).for_each(|i| {
                   self.queue.push_back(Point::new(i as f32,self.rng.gen::<f32>() * 400.0));
                });
                self.entities.push(Entity {
                    pos: Point::new(0.0, 0.0),
                    center: Point::new(core.config.width as f32 / 2.0, core.config.height as f32 / 2.0),
                    radius: 256.0,
                    angle: 0.0,
                    speed: 1.0,
                });
                self.entities.push(Entity {
                    pos: Point::new(0.0, 0.0),
                    center: Point::new(core.config.width as f32 / 2.0, core.config.height as f32 / 2.0),
                    radius: 256.0,
                    angle: std::f32::consts::PI,
                    speed: 1.0,
                });
            }
            Event::Resized(_, _) => {}
            Event::Fps(_) => {}
        }
    }
}