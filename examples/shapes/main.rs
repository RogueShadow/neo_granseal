use std::f32::consts::TAU;
use neo_granseal::prelude::*;
use neo_granseal::events::Event;
use neo_granseal::mesh::FillStyle;
use neo_granseal::shape_pipeline::{ShapeGfx};
use neo_granseal::util::{Color, Point};

fn main() {
    start(Shapes {},GransealGameConfig::new())
}

struct Shapes {}
impl NeoGransealEventHandler for Shapes {
    fn event(&mut self, core: &mut NGCore, e: Event) {
        match e {
            Event::Draw => {
                core.set_title(format!("Shapes Showcase Fps: {}", core.state.fps));
                let time = core.timer.elapsed().as_secs_f32();
                let angle = (time) % (TAU);
                let zero = Point::new(0, 0);
                let size = Point::new(64,64);
                let step = Point::new(96, 0);
                let mut g = ShapeGfx::new(core);
                g.push_state();
                let draw_shapes = |g: &mut ShapeGfx|{
                    g.rect(zero, size);
                    g.translate(step);
                    g.oval(zero, size / 2.0, 4.0);
                    g.translate(step);
                    g.arc(zero, Point::new(32.0,32.0), angle/2.0 - tau/4.0, angle, 1.0);
                    g.translate(step);
                    g.set_fill(false);
                    g.rect(zero, size);
                    g.translate(step);
                    g.oval(zero, size / 2.0, 4.0);
                    g.translate(step);
                    g.arc(zero, Point::new(32.0,32.0), angle/2.0 - tau/4.0, angle, 1.0);
                    g.translate(step);
                    g.line(Point::ZERO, size);
                };
                g.translate(size);
                draw_shapes(&mut g);
                g.pop_state();
                g.set_fill_style(FillStyle::FadeDown(Color::SADDLE_BROWN,Color::CORNFLOWER_BLUE));
                g.set_position(zero + size + Point::new(0,96));
                g.set_rotation_origin(size / 2.0);
                g.set_rotation(3.14/4.0);
                g.set_line_thickness(8.0);
                draw_shapes(&mut g);
                g.finish();
            }
            _ => {}
        }
    }
}