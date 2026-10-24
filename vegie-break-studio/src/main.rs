// Toolbar design spec (see toolbar.md):
// - Modern neon gradient style (cyan-to-purple gradient strip)
// - Based on Logic Pro X control bar layout
// - No footer bar; only top bar
// - FL Studio toolbar elements to be added in future updates
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowAttributes;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = event_loop.create_window(
        WindowAttributes::default()
            .with_title("Vegie Break Studio")
            .with_inner_size(LogicalSize::new(1280, 720))
    ).unwrap();

    event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                control_flow.exit();
            }
            _ => (),
        }
    }).unwrap();
}
