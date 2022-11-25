use neo_granseal::mesh::MeshBuilder;
use neo_granseal::prelude::*;

fn main() {
    start(Box {},GransealGameConfig::new())
}

struct Box {}

impl NeoGransealEventHandler for Box {
    fn event(&mut self, core: &mut NGCore, event: Event) {
        match event {
            Event::Draw => {
                let mut mb = MeshBuilder::new();
                mb.rect(Vec2::new(128.0,256.0));
                let mut g = ShapeGfx::new(core);
                g.draw_mesh(&mb.build(),Vec2::new(128.0,128.0));
                g.finish();
            }
            _ => {}
        }
    }
}