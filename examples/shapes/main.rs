use neo_granseal::events::Event;
use neo_granseal::mesh::{FillStyle, MeshBuilder};
use neo_granseal::prelude::*;
use neo_granseal::shape_pipeline::ShapeGfx;
use neo_granseal::util::Color;
use std::f32::consts::{PI, TAU};
use neo_granseal::MSAA;

fn main() {
    start(
        Shapes {},
        GransealGameConfig::new().vsync(false).size(1024, 400).msaa(MSAA::Enable8x),
    )
}

struct Shapes {}
impl NeoGransealEventHandler for Shapes {
    fn event(&mut self, core: &mut NGCore, e: Event) {
        if e == Event::Draw {
            core.set_title(format!("Shapes Showcase Fps: {}", core.state.fps));
            let time = core.timer.elapsed().as_secs_f32();
            let angle = (time) % (TAU);
            let zero = Vec2::new(0, 0);
            let size = Vec2::new(64, 64);
            let step = Vec2::new(96, 0);
            let mut mb = MeshBuilder::default();
            mb.set_style(FillStyle::Solid(Color::TEAL));

            mb.push();
            mb.move_cursor(size);
            mb.rect(size);
            mb.move_cursor(step);
            mb.rounded_rect(size, 16.0);
            mb.move_cursor(step);
            mb.oval(size / 2.0);
            mb.move_cursor(step);
            mb.arc(Vec2::new(32.0, 32.0), angle / 2.0 - TAU / 4.0, angle);
            mb.move_cursor(step);
            mb.set_filled(false);
            mb.rect(size);
            mb.move_cursor(step);
            mb.rounded_rect(size, 16.0);
            mb.move_cursor(step);
            mb.oval(size / 2.0);
            mb.move_cursor(step);
            mb.arc(Vec2::new(32.0, 32.0), angle / 2.0 - TAU / 4.0, angle);
            mb.move_cursor(step);
            mb.line(Vec2::ZERO, size);
            mb.pop();
            mb.set_style(FillStyle::FadeDown(
                Color::SADDLE_BROWN,
                Color::CORNFLOWER_BLUE,
            ));
            mb.move_cursor(zero + size + Vec2::new(0, 96));
            mb.set_rotation(PI / 4.0, size / 2.0);
            mb.set_thickness(8.0);
            mb.rect(size);
            mb.move_cursor(step);
            mb.rounded_rect(size, 16.0);
            mb.move_cursor(step);
            mb.oval(size / 2.0);
            mb.move_cursor(step);
            mb.arc(Vec2::new(32.0, 32.0), angle / 2.0 - TAU / 4.0, angle);
            mb.move_cursor(step);
            mb.set_filled(false);
            mb.rect(size);
            mb.move_cursor(step);
            mb.rounded_rect(size, 16.0);
            mb.move_cursor(step);
            mb.oval(size / 2.0);
            mb.move_cursor(step);
            mb.arc(Vec2::new(32.0, 32.0), angle / 2.0 - TAU / 4.0, angle);
            mb.move_cursor(step);
            mb.line(Vec2::ZERO, size);
            let mut g = ShapeGfx::new(core);
            g.draw_mesh(&mb.build(), Vec2::ZERO);
            g.finish();
        }
    }
}
