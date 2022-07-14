extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

extern crate winit;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// #[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, JsValue};

// #[cfg(target_arch = "wasm32")]
use web_sys::{console, Window};

use crate::*;

pub fn start() {
    use winit::platform::web::WindowExtWebSys;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let canvas = window.canvas();

    let web_win = web_sys::window().unwrap();
    let document = web_win.document().unwrap();
    let body = document.body().unwrap();

    canvas.style().set_css_text("background-color: lavender;");
    canvas.set_width(720);
    canvas.set_height(480);
    body.append_child(&canvas).unwrap();

    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");
    trace!("trace");

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

// #[cfg(target_arch = "wasm32")]
pub mod wasm {
    use super::*;

    #[wasm_bindgen(start)]
    pub fn run() {
        #[allow(clippy::main_recursion)]
        super::start();
    }
}
