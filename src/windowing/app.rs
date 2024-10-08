use std::sync::Arc;
use wgpu::SurfaceError;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use crate::windowing::state_mngr::State;
use futures::executor;

#[derive(Default)]
struct App<'a> {
    window: Option<Arc<Window>>,
    state: Option<State<'a>>,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = match event_loop.create_window(Window::default_attributes()) {
            Ok(window) => Arc::new(window),
            Err(e) => {
                eprintln!("Error creating window: {}", e);
                return;
            }
        };

        window.request_redraw();

        let state_future = State::new(Arc::clone(&window));
        let state = executor::block_on(executor::block_on(state_future));
        self.state = Some(state);
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                if let Some(state) = self.state.as_mut() {
                    match state.render() {
                        Ok(_) => {}
                            Err(SurfaceError::Lost) => state.resize(state.size),
                        Err(SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => eprintln!("{:?}", e)
                    }
                    state.window.request_redraw();
                }
            }
            _ => (),
        }
    }
}


pub(crate) fn open_window(flow: ControlFlow) {
    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(e) => {
            eprintln!("Failed to create event loop: {:?}", e);
            return;
        }
    };

    event_loop.set_control_flow(flow);

    let mut app = App::default();
    if let Err(e) = event_loop.run_app(&mut app) {
        eprintln!("Failed to run app: {:?}", e);
    }
}