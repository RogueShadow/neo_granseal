use neo_granseal::prelude::*;

fn main() {
    start(Box {},GransealGameConfig::new())
}

struct Box {}
impl NeoGransealEventHandler for Box {
    fn event(&mut self, core: &mut NGCore, e: Event) {
        match e {
            Event::Draw => {
                let mut g = ShapeGfx::new(core);
                g.rect(Point::new(128.0,128.0),Point::new(128.0,128.0));
                g.finish();
            }
            _ => {}
        }
    }
}