extern crate libc;

#[cfg(target_os = "linux")]
use ::system::xcb::{
    xcb_cw_t,
    xcb_flush,
    xcb_setup_t,
    xcb_connect,
    xcb_window_t,
    xcb_get_setup,
    xcb_disconnect,
    xcb_map_window,
    xcb_generate_id,
    xcb_screen_next,
    xcb_intern_atom,
    xcb_prop_mode_t,
    xcb_connection_t,
    xcb_event_mask_t,
    xcb_create_window,
    xcb_destroy_window,
    xcb_window_class_t,
    xcb_config_window_t,
    xcb_generic_error_t,
    xcb_change_property,
    xcb_configure_window,
    xcb_intern_atom_reply,
    xcb_screen_iterator_t,
    xcb_setup_roots_iterator,

    XCB_COPY_FROM_PARENT,
};

use ::system::vulkan::{
    VkRect2D,
    VkResult,
    VkExtent2D,
    VkOffset2D,
    VkSurfaceKHR,
    VkStructureType,
    vkDestroySurfaceKHR,
    VkAllocationCallbacks,
};

use ::system::vulkan_xcb::{
    vkCreateXcbSurfaceKHR,
    VkXcbSurfaceCreateInfoKHR,
};

use std::mem::zeroed;
use std::default::Default;
use std::ffi::CString;
use std::mem::transmute;
use std::os::raw::{
    c_int,
    c_uint,
    c_char,
};
use std::sync::{
    Arc,
    RwLock,
};

use super::device::Device;

#[cfg(target_os = "linux")]
struct OsWindow {
    connection: *mut xcb_connection_t,
    window: xcb_window_t
}

#[cfg(target_os = "linux")]
impl OsWindow {
    fn new(window: &mut Window, width: u32, height: u32) -> Self {
        let setup: *const xcb_setup_t;
        let mut iter: xcb_screen_iterator_t;
        let mut screen = 0 as c_int;
        let xcb_connection = unsafe { xcb_connect(0 as *const c_char, &mut screen as *mut c_int) };
        if xcb_connection == (0 as *mut xcb_connection_t) {
            panic!("Cannot find a compatible Vulkan ICD.");
        }
        setup = unsafe {xcb_get_setup(xcb_connection) };
        iter = unsafe { xcb_setup_roots_iterator(setup) };
        let _ = setup;
        for _ in 0..screen {
            unsafe { xcb_screen_next(&mut iter as *mut xcb_screen_iterator_t); }
        }
        let xcb_screen = iter.data;
        let _ = iter;
        let dimensions = VkRect2D {
            offset: VkOffset2D {
                x: 0,
                y: 0,
            },
            extent: VkExtent2D {
                width: width,
                height: height
            },
        };
        let value_mask: c_uint;
        let mut value_list = [0 as c_uint; 32];
        let xcb_window = unsafe { xcb_generate_id(xcb_connection) };
        value_mask = (xcb_cw_t::XCB_CW_BACK_PIXEL as c_uint) |
            (xcb_cw_t::XCB_CW_EVENT_MASK as c_uint);
        value_list[0] = unsafe { (*xcb_screen).black_pixel };
        value_list[1] = (xcb_event_mask_t::XCB_EVENT_MASK_KEY_RELEASE as c_uint) |
            (xcb_event_mask_t::XCB_EVENT_MASK_EXPOSURE as c_uint);
        unsafe {
            xcb_create_window(
                xcb_connection, XCB_COPY_FROM_PARENT as u8, xcb_window, (*xcb_screen).root,
                dimensions.offset.x as i16, dimensions.offset.y as i16,
                dimensions.extent.width as u16, dimensions.extent.height as u16, 0,
                xcb_window_class_t::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16, (*xcb_screen).root_visual,
                value_mask, value_list.as_ptr() as *const u32);
        }
        let wm_protocols = CString::new("WM_PROTOCOLS").unwrap();
        let cookie = unsafe { xcb_intern_atom(xcb_connection, 1, 12, wm_protocols.as_ptr()) };
        let reply = unsafe { xcb_intern_atom_reply(
            xcb_connection, cookie, 0 as *mut *mut xcb_generic_error_t) };
        let wm_delete_window = CString::new("WM_DELETE_WINDOW").unwrap();
        let cookie2 = unsafe { xcb_intern_atom(xcb_connection, 0, 16, wm_delete_window.as_ptr()) };
        let xcb_atom_window_reply = unsafe { xcb_intern_atom_reply(
            xcb_connection, cookie2, 0 as *mut *mut xcb_generic_error_t) };
        unsafe {
            xcb_change_property(
                xcb_connection, xcb_prop_mode_t::XCB_PROP_MODE_REPLACE as u8, xcb_window,
                (*reply).atom, 4, 32, 1, transmute(&(*xcb_atom_window_reply).atom));
        }
        unsafe { libc::free(reply as *mut libc::c_void) };
        unsafe { xcb_map_window(xcb_connection, xcb_window) };
        let coords = [100 as c_uint; 2];
        unsafe {
            xcb_configure_window(
                xcb_connection, xcb_window,
                (xcb_config_window_t::XCB_CONFIG_WINDOW_X as u16) |
                    (xcb_config_window_t::XCB_CONFIG_WINDOW_Y as u16),
                coords.as_ptr() as *const u32);
        }
        unsafe {
            xcb_flush(xcb_connection);
        }
        let create_info = VkXcbSurfaceCreateInfoKHR {
            sType: VkStructureType::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
            connection: xcb_connection,
            window: xcb_window,
            ..VkXcbSurfaceCreateInfoKHR::default()
        };
        let dev = window.device.read().unwrap();
        let ins = dev.instance.read().unwrap();
        vulkan_check!(vkCreateXcbSurfaceKHR(ins.vk_instance,
            &create_info as *const VkXcbSurfaceCreateInfoKHR, 0 as *const VkAllocationCallbacks,
            &mut window.surface as *mut VkSurfaceKHR));
        OsWindow {
            connection: xcb_connection,
            window: xcb_window,
        }
    }
}

#[cfg(target_os = "linux")]
impl Default for OsWindow {
    fn default() -> Self {
        unsafe {
            zeroed()
        }
    }
}

#[cfg(target_os = "linux")]
impl Drop for OsWindow {
    fn drop(&mut self) {
        if self.connection == 0 as *mut xcb_connection_t {
            return;
        }
        unsafe { xcb_destroy_window(self.connection, self.window); }
        unsafe { xcb_disconnect(self.connection); }
        self.connection = 0 as *mut xcb_connection_t;
    }
}


pub struct Window {
    device: Arc<RwLock<Device>>,
    window: OsWindow,
    surface: VkSurfaceKHR,
}

impl Window {
    pub fn new(device: Arc<RwLock<Device>>) -> Self {
        let mut window = Window {
            device: device,
            window: OsWindow::default(),
            surface: 0 as VkSurfaceKHR,
        };
        window.window = OsWindow::new(&mut window, 900, 500);
        window
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        let dev = self.device.read().unwrap();
        let ins = dev.instance.read().unwrap();
        unsafe {
            vkDestroySurfaceKHR(ins.vk_instance, self.surface, 0 as *const VkAllocationCallbacks);
        }
        self.window = OsWindow::default();
    }
}