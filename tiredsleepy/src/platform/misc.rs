#[cfg(target_arch = "wasm32")]
extern crate js_sys;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;

#[cfg(target_arch = "wasm32")]
extern crate web_sys;

extern crate winit;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn start() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;

        let canvas = window.canvas();

        let web_win = web_sys::window().unwrap();
        let document = web_win.document().unwrap();
        let body = document.body().unwrap();

        canvas.set_width(720);
        canvas.set_height(480);
        body.append_child(&canvas).unwrap();
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        // debug!("{event:?}");

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}

#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn run() {
        #[allow(clippy::main_recursion)]
        super::start();
    }
}
