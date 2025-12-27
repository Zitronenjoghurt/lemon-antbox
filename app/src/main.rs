use crate::app::App;
use lemon_antbox_core::simulation::settings::SimulationSettings;
use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod app;
mod gfx;
mod ui;

const WIDTH: u16 = 640;
const HEIGHT: u16 = 360;
const TRIBE_COUNT: u8 = 2;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Lemon Antbox")
            .with_inner_size(LogicalSize::new(WIDTH * 3, HEIGHT * 3))
            .with_min_inner_size(LogicalSize::new(WIDTH, HEIGHT))
            .with_resize_increments(LogicalSize::new(WIDTH, HEIGHT))
            .build(&event_loop)
            .unwrap(),
    );

    let settings = SimulationSettings {
        width: WIDTH,
        height: HEIGHT,
        tribe_count: TRIBE_COUNT,
        ..Default::default()
    };
    let mut app = App::new(window.clone(), settings);

    event_loop
        .run(move |event, target| {
            app.handle_event(event, target);
        })
        .unwrap();
}
