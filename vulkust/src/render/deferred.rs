use super::buffer::DynamicBuffer;
use super::descriptor::Set as DescriptorSet;
use super::engine::GraphicApiEngine;
use super::scene::Manager as SceneManager;
use std::sync::{Arc, RwLock};
use std::mem::size_of;

#[repr(C)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Uniform {
    samples_count: u32,
	inverse_samples_count: f32,
	window_width: f32,
	window_height: f32,
	pixel_x_step: f32,
	pixel_y_step: f32,
}

impl Uniform {
    pub fn new(samples_count: u32, window_width: f32, window_height: f32) -> Self {
        Uniform {
            samples_count,
            inverse_samples_count: 1f32 / (samples_count as f32),
            window_width,
            window_height,
            pixel_x_step: 2f32 / window_width,
            pixel_y_step: 2f32 / window_height,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Deferred {
    uniform: Uniform,
    uniform_buffer: Arc<RwLock<DynamicBuffer>>,
    descriptor_set: Arc<DescriptorSet>,
}

impl Deferred {
    pub fn new(gapi_engine: &GraphicApiEngine, scene_manager: &SceneManager) -> Self {
        let (w, h) = gapi_engine.g_framebuffer.get_dimensions();
        let uniform = Uniform::new(gapi_engine.samples_count as u32, w as f32, h as f32);
        let uniform_buffer = Arc::new(RwLock::new(
            vxresult!(gapi_engine.buffer_manager.write())
                .create_dynamic_buffer(size_of::<Uniform>() as isize),
        ));
        let mut descriptor_manager = vxresult!(gapi_engine.descriptor_manager.write());
        let sampler = &gapi_engine.sampler;
        let mut texture_manager = vxresult!(scene_manager.texture_manager.write());
        let mut textures = Vec::new();
        for v in vxunwrap!(&gapi_engine.g_render_pass.views) {
            textures.push(texture_manager.create_2d_with_view_sampler(
                v.clone(), sampler.clone()));
        }
        textures.shrink_to_fit();
        let descriptor_set = descriptor_manager.create_deferred_set(uniform_buffer.clone(), textures);
        let descriptor_set = Arc::new(descriptor_set);
        
        Deferred {
            uniform,
            uniform_buffer,
            descriptor_set,
        }
    }
}