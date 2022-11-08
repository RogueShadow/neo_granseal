use std::ops::Rem;
use neo_granseal::core::{NGCommand, NGCore};
use neo_granseal::events::Event;
use neo_granseal::{GransealGameConfig, NeoGransealEventHandler, start};
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
                core.cmd(NGCommand::SetTitle(format!("Shapes Showcase Fps: {}", core.state.fps)));
                let tau = std::f32::consts::TAU;
                let time = core.timer.elapsed().as_secs_f32();
                let angle = (time).rem(tau);
                let thickness = 1.0 + ((time * 2.0).sin() * 16.0).abs();
                let rotation = time;
                let styles = [
                    FillStyle::Solid(Color::TEAL),
                    FillStyle::FadeLeft(Color::TEAL, Color::DARK_OLIVE_GREEN),
                    FillStyle::FadeDown(Color::TEAL, Color::YELLOW),
                    FillStyle::Corners(Color::RED, Color::LIME, Color::BLUE, Color::BLACK),
                    FillStyle::FadeLeft(Color::TRANSPARENT, Color::TEAL),
                ];
                let zero = Point::new(0.0, 0.0);
                let size = Point::new(64.0, 64.0);
                let step = Point::new(96.0, 0.0);
                let mut g = ShapeGfx::new(core);
                {
                    g.set_rotation_origin(Point::new(32,32));
                    g.set_rotation(rotation);

                    g.set_line_thickness(thickness);
                    g.set_fill_style(styles[0]);
                    g.translate(size);
                    g.rect(zero, size);
                    g.translate(step);
                    g.circle(zero, size / 2.0, 4.0);
                    g.translate(step);
                    g.arc(zero, Point::new(32.0,32.0), angle - tau/4.0, angle, 1.0);
                    g.translate(step);
                    g.set_fill(false);
                    g.rect(zero, size);
                    g.translate(step);
                    g.circle(zero, size / 2.0, 4.0);
                    g.translate(step);
                    g.push_state();

                    g.line(Point::new(-32.0, -32.0), size / 2.0);
                    g.pop_state();
                    g.translate(step);
                    g.arc(zero, Point::new(32.0,32.0), angle - tau/4.0, angle, 1.0);

                    g.set_fill(true);
                    g.set_fill_style(styles[1]);
                    g.set_position(zero);
                    g.translate(size);
                    g.translate(Point::new(0.0, 96.0));
                    g.rect(zero, size);
                    g.translate(step);
                    g.circle(zero, size / 2.0, 4.0);
                    g.translate(step);
                    g.arc(zero, Point::new(32.0,32.0), angle - tau/4.0, angle, 1.0);
                    g.translate(step);
                    g.set_fill(false);
                    g.rect(zero, size);
                    g.translate(step);
                    g.circle(zero, size / 2.0, 4.0);
                    g.translate(step);
                    g.line(Point::new(-32.0, -32.0), size / 2.0);
                    g.translate(step);
                    g.arc(zero, Point::new(32.0,32.0), angle - tau/4.0, angle, 1.0);

                    g.set_fill(true);
                    g.set_fill_style(styles[2]);
                    g.set_position(zero);
                    g.translate(size);
                    g.translate(Point::new(0.0, 96.0 * 2.0));
                    g.rect(zero, size);
                    g.translate(step);
                    g.circle(zero, size / 2.0, 4.0);
                    g.translate(step);
                    g.arc(zero, Point::new(32.0,32.0), angle - tau/4.0, angle, 1.0);
                    g.translate(step);
                    g.set_fill(false);
                    g.rect(zero, size);
                    g.translate(step);
                    g.circle(zero, size / 2.0, 4.0);
                    g.translate(step);
                    g.line(Point::new(-32.0, -32.0), size / 2.0);
                    g.translate(step);
                    g.arc(zero, Point::new(32.0,32.0), angle - tau/4.0, angle, 1.0);

                    g.set_fill(true);
                    g.set_fill_style(styles[3]);
                    g.set_position(zero);
                    g.translate(size);
                    g.translate(Point::new(0.0, 96.0 * 3.0));
                    g.rect(zero, size);
                    g.translate(step);
                    g.circle(zero, size / 2.0, 4.0);
                    g.translate(step);
                    g.arc(zero, Point::new(32.0,32.0), angle - tau/4.0, angle, 1.0);
                    g.translate(step);
                    g.set_fill(false);
                    g.rect(zero, size);
                    g.translate(step);
                    g.circle(zero, size / 2.0, 4.0);
                    g.translate(step);
                    g.line(Point::new(-32.0, -32.0), size / 2.0);
                    g.translate(step);
                    g.arc(zero, Point::new(32.0,32.0), angle - tau/4.0, angle, 1.0);

                    g.set_fill(true);
                    g.set_fill_style(styles[4]);
                    g.set_position(zero);
                    g.translate(size);
                    g.translate(Point::new(0.0, 96.0 * 4.0));
                    g.rect(zero, size);
                    g.translate(step);
                    g.circle(zero, size / 2.0, 4.0);
                    g.translate(step);
                    g.arc(zero, Point::new(32.0,32.0), angle - tau/4.0, angle, 1.0);
                    g.translate(step);
                    g.set_fill(false);
                    g.rect(zero, size);
                    g.translate(step);
                    g.circle(zero, size / 2.0, 4.0);
                    g.translate(step);
                    g.line(Point::new(-32.0, -32.0), size / 2.0);
                    g.translate(step);
                    g.arc(zero, Point::new(32.0,32.0), angle - tau/4.0, angle, 1.0);
                }
                g.finish();
            }
            _ => {}
        }
    }
}