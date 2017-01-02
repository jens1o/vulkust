//pub mod buffer;
//pub mod command;
//pub mod device;
//pub mod fence;
//pub mod image;
pub mod instance;
//pub mod swapchain;
//pub mod window;

//use std;
use std::sync::{
    Arc,
};

pub struct Driver {
    pub instance: Arc<instance::Instance>,
//    pub device: Arc<device::Device>,
//    pub cmd_pool: Arc<command::pool::Pool>,
//    pub window: Arc<window::Window>,
//    pub swapchain: Arc<swapchain::Swapchain>,
}

impl Driver {
    pub fn initialize(&mut self) {
        self.instance = Arc::new(instance::Instance::new());
//        let dev = Arc::new(device::Device::new(ins.clone()));
//        let cmd_pool = Arc::new(command::pool::Pool::new(
//            dev.clone(), dev.graphics_family_index));
//        let win = Arc::new(window::Window::new(dev.clone()));
//        let swp = Arc::new(swapchain::Swapchain::new(win.clone()));
    }
}
