use super::xcb;
use super::super::super::core::application::Application as CoreApp;
use super::super::super::render::engine::Engine as RenderEngine;
use std::ptr::{
    null,
    null_mut,
};

pub struct Application <App, RenderEng> where App: CoreApp, RenderEng: RenderEngine {
	connection: *mut xcb::xcb_connection_t,
    screen: *mut xcb_screen_t,
    window: *mut xcb_window_t,
    atom_wm_delete_window: *mut xcb_intern_atom_reply_t,
   	core_app: App,
    render_engine: RenderEng,
}

impl Application <App, RenderEng> where App: CoreApp, RenderEng: RenderEngine {
	fn new(a: App, r: RenderEng) -> Self {
		Application {
            connection: null_ptr,
            screen:  null_ptr,
            window: null_ptr,
            atom_wm_delete_window: null_ptr,
           	core_app: a,
            render_engine: r,
		}
	}
}
