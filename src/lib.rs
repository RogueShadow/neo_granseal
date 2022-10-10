use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop;
use winit::event_loop::ControlFlow;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn build() {
    let event_loop = event_loop::EventLoop::new();
    let builder = winit::window::WindowBuilder::new();

    let window = builder
        .with_title("Neo Granseal")
        .with_resizable(false)
        .with_inner_size(PhysicalSize::new(800,600))
        .build(&event_loop)
        .expect("Failed to build window");


    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
