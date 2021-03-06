use super::super::super::core::application::Application as CoreAppTrait;
use super::super::super::render::engine::Engine as RenderEngine;
use super::super::apple;
use std::mem::transmute;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::sync::{Arc, RwLock, Weak};

pub struct Application {
    pub core_app: Arc<RwLock<CoreAppTrait>>,
    pub itself: Option<Weak<RwLock<Application>>>,
    pub view: *mut c_void,
    pub renderer: Option<Arc<RwLock<RenderEngine>>>,
}

impl Application {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>) -> Self {
        Application {
            core_app,
            itself: None,
            view: null_mut(),
            renderer: None,
        }
    }

    pub fn set_itself(&mut self, itself: Weak<RwLock<Application>>) {
        self.itself = Some(itself);
    }

    pub fn update(&self) {
        vxresult!(vxunwrap!(self.renderer).read()).update();
    }

    pub fn get_window_aspect_ratio(&self) -> f32 {
        let view: apple::Id = unsafe { transmute(self.view) };
        let frame: apple::NSRect = unsafe { msg_send![view, frame] };
        frame.size.width as f32 / frame.size.height as f32
    }
}

impl Drop for Application {
    fn drop(&mut self) {}
}

#[no_mangle]
pub extern "C" fn vulkust_deallocate(context: *mut c_void) {
    let os_app: *mut Arc<RwLock<Application>> = unsafe { transmute(context) };
    unsafe {
        let _ = Box::from_raw(os_app);
    }
    vxlogi!("Reached");
}

#[no_mangle]
pub extern "C" fn vulkust_set_view(context: *mut c_void, view: *mut c_void) {
    let os_app: &'static Arc<RwLock<Application>> = unsafe { transmute(context) };
    vxresult!(os_app.write()).view = view;
    let core_app = vxresult!(os_app.read()).core_app.clone();
    let renderer = Arc::new(RwLock::new(RenderEngine::new(core_app.clone(), os_app)));
    let renderer_w = Arc::downgrade(&renderer);
    vxresult!(renderer.write()).set_myself(renderer_w);
    vxresult!(os_app.write()).renderer = Some(renderer.clone());
    let mut core_app = vxresult!(core_app.write());
    core_app.set_os_app(os_app.clone());
    core_app.set_renderer(renderer);
    core_app.initialize();
    vxlogi!("Reached");
}

#[no_mangle]
pub extern "C" fn vulkust_render(context: *mut c_void) {
    let os_app: &'static Arc<RwLock<Application>> = unsafe { transmute(context) };
    let os_app = vxresult!(os_app.read());
    os_app.update();
}
