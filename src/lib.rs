mod windowing;

use winit::event_loop::ControlFlow;

pub fn run(flow: ControlFlow) {
    env_logger::init();
    windowing::open_window(flow);
}