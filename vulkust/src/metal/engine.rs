use std::ptr::null_mut;
use std::mem::size_of;
use super::super::core::application::ApplicationTrait;
use super::super::math::matrix::{Mat4x4, Mat3x3};
use super::super::math::vector::Vec3;
use super::super::system::os::OsApplication;
use super::super::render::engine::EngineTrait;
use super::super::system::metal as mtl;

pub struct Engine<CoreApp> where CoreApp: ApplicationTrait {
    pub core_app: *mut CoreApp,
    pub os_app: *mut OsApplication<CoreApp>,
}

const MAX_BUFFERS_COUNT: mtl::NSUInteger = 3;

#[repr(C)]
#[derive(Debug)]
pub struct Uniforms {
    pub projectionMatrix: Mat4x4<f32>,
    pub viewMatrix: Mat4x4<f32>,
    pub materialShininess: f32,
    pub modelViewMatrix: Mat4x4<f32>,
    pub normalMatrix: Mat3x3<f32>,
    pub ambientLightColor: Vec3<f32>,
    pub directionalLightDirection: Vec3<f32>,
    pub directionalLightColor: Vec3<f32>,
}

impl<CoreApp> EngineTrait<CoreApp> for Engine<CoreApp> where CoreApp: ApplicationTrait {
    fn new() -> Self {
        Engine {
            core_app: null_mut(),
            os_app: null_mut(),
        }
    }

    fn set_core_app(&mut self, c: *mut CoreApp) {
        self.core_app = c;
    }

    fn set_os_app(&mut self, o: *mut OsApplication<CoreApp>) {
        self.os_app = o;
    }

    fn initialize(&mut self) {
        let device = unsafe { (*self.os_app).metal_device };
        let asset_manager = unsafe {&mut (*self.os_app).asset_manager };
        let shader = asset_manager.get_shader(1, self.os_app);
        let uniform_buffer_size: mtl::NSUInteger =
            ((size_of::<Uniforms>() as mtl::NSUInteger & !0xFF) + 0x100) * MAX_BUFFERS_COUNT;
        let dynamic_uniform_buffer: mtl::Id = unsafe {
            msg_send![device, newBufferWithLength:uniform_buffer_size
                options:mtl::RESOURCE_STORAGE_MODE_SHARED] };

    }

    fn update(&mut self) {

    }

    fn terminate(&mut self) {

    }
}
