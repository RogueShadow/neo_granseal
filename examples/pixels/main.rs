use std::thread::Builder;
use log::info;
use pollster::FutureExt;
use neo_granseal::{events::Event, start, GransealGameConfig, NeoGransealEventHandler, shape_pipeline};
use neo_granseal::core::NGCommand::AddPipeline;
use neo_granseal::core::NGCore;

fn main() {
    start(Game::new(), GransealGameConfig::new());
}

struct Game {}
impl Game {
    fn new() -> Self {
        Self {}
    }
}

impl NeoGransealEventHandler for Game {
    fn event(&mut self, core: &mut NGCore, e: Event) {
        match e {
            Event::KeyEvent { .. } => {}
            Event::MouseButton { .. } => {}
            Event::MouseMoved { .. } => {}
            Event::Draw => {}
            Event::Update(_) => {}
            Event::Load => {
                core.cmd(AddPipeline(Box::new(shape_pipeline::SimpleShapeRenderPipeline::new(&core))));
            }
            Event::Resized(_, _) => {}
        }
    }
}

