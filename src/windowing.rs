use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // draw


                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}


pub fn open_window(flow: ControlFlow) {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(flow);

    let mut app = App::default();
    event_loop.run_app(&mut app).expect("TODO: panic message");
}