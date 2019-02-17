use super::super::super::super::core::types::Real;
use super::super::super::buffer::Dynamic as DynamicBuffer;
use super::super::super::camera::Camera;
use super::super::super::command::Buffer as CmdBuffer;
use super::super::super::descriptor::Set as DescriptorSet;
use super::super::super::engine::Engine;
use super::super::super::framebuffer::Framebuffer;
use super::super::super::gapi::GraphicApiEngine;
use super::super::super::image::{AttachmentType, Format, View as ImageView};
use super::super::super::pipeline::{Pipeline, PipelineType};
use super::super::super::render_pass::RenderPass;
use super::super::super::sampler::Filter;
use super::super::super::scene::Scene;
use super::super::super::sync::Semaphore;
use super::super::super::texture::Texture;
use super::{Base, LinkId, Node};
use cgmath;
use std::mem::size_of;
use std::sync::{Arc, Mutex, RwLock};

const INPUT_LINKS_NAMES: [&str; 6] = [
    super::POSITION_NAME_LINK,
    super::NORMAL_NAME_LINK,
    super::ALBEDO_NAME_LINK,
    super::DEPTH_NAME_LINK,
    super::OCCLUSION_NAME_LINK,
    super::ACCUMULATED_SHADOWS_NAME_LINK,
];

const INPUT_LINKS_IDS: [LinkId; 6] = [
    super::POSITION_LINK,
    super::NORMAL_LINK,
    super::ALBEDO_LINK,
    super::DEPTH_LINK,
    super::OCCLUSION_LINK,
    super::ACCUMULATED_SHADOWS_LINK,
];

#[cfg_attr(debug_mode, derive(Debug))]
struct FrameData {
    pri_cmd: CmdBuffer,
    sec_cmd: CmdBuffer,
    semaphore: Arc<Semaphore>,
}

impl FrameData {
    fn new(geng: &GraphicApiEngine) -> Self {
        let pri_cmd = geng.create_primary_command_buffer_from_main_graphic_pool();
        let sec_cmd = geng.create_secondary_command_buffer_from_main_graphic_pool();
        let semaphore = Arc::new(geng.create_semaphore());
        Self {
            pri_cmd,
            sec_cmd,
            semaphore,
        }
    }
}

#[repr(C)]
#[cfg_attr(debug_mode, derive(Debug))]
struct Uniform {
    pixel_step: cgmath::Vector4<Real>,
}

impl Uniform {
    pub fn new(window_width: Real, window_height: Real) -> Self {
        Self {
            pixel_step: cgmath::Vector4::new(1f32 / window_width, 1f32 / window_height, 0.0, 0.0),
        }
    }
}

/// This struct is gonna be created for each instance
#[cfg_attr(debug_mode, derive(Debug))]
struct RenderData {
    frames_data: Vec<FrameData>,
    input_textures: Vec<Arc<RwLock<Texture>>>,
    uniform: Uniform,
    uniform_buffer: DynamicBuffer,
    descriptor_set: Arc<DescriptorSet>,
}

impl RenderData {
    fn new(geng: &GraphicApiEngine) -> Self {
        let frames_count = geng.get_frames_count();
        let mut frames_data = Vec::with_capacity(frames_count);
        for _ in 0..frames_count {
            frames_data.push(FrameData::new(geng));
        }
        let (w, h) = geng.get_current_framebuffer().get_dimensions();
        let uniform = Uniform::new(w as Real, h as Real);
        let uniform_buffer = vxresult!(geng.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        Self {
            frames_data,
            uniform,
            uniform_buffer,
            input_textures,
        }
    }
}

/// This struct is gonna be created for each instance
#[cfg_attr(debug_mode, derive(Debug))]
#[derive(Clone)]
struct SharedData {
    texture: Arc<RwLock<Texture>>,
    render_pass: Arc<RenderPass>,
    framebuffer: Arc<Framebuffer>,
    pipeline: Arc<Pipeline>,
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct DeferredPbr {
    base: Base,
    shared_data: SharedData,
    render_data: RenderData,
}

impl DeferredPbr {
    pub fn new(eng: &Engine) -> Self {
        let geng = eng.get_gapi_engine();
        let geng = vxresult!(geng.read());
        let dev = geng.get_device();
        let memmgr = geng.get_memory_manager();
        let buffer = Arc::new(ImageView::new_surface_attachment(
            dev.clone(),
            memmgr,
            Format::RgbaFloat,
            AttachmentType::ColorGBuffer,
        ));
        let sampler = vxresult!(geng.get_sampler_manager().write()).load(Filter::Nearest);
        let texture = vxresult!(eng.get_asset_manager().get_texture_manager().write())
            .create_2d_with_view_sampler(buffer.clone(), sampler);
        let render_pass = Arc::new(RenderPass::new(vec![buffer.clone()], true, true));
        let framebuffer = Arc::new(Framebuffer::new(vec![buffer], render_pass.clone()));
        let pipeline = vxresult!(geng.get_pipeline_manager().write()).create(
            render_pass.clone(),
            PipelineType::DeferredPbr,
            eng.get_config(),
        );
        let shared_data = SharedData {
            texture,
            framebuffer,
            render_pass,
            pipeline,
        };
        let render_data = RenderData::new(&geng);
        let base = Base::new(
            super::DEFERRED_PBR_NODE,
            "deferred-pbr".to_string(),
            {
                let mut names = Vec::with_capacity(INPUT_LINKS_NAMES.len());
                for l in &INPUT_LINKS_NAMES {
                    names.push(l.to_string());
                }
                names
            },
            {
                let mut ids = Vec::with_capacity(INPUT_LINKS_IDS.len());
                for l in &INPUT_LINKS_IDS {
                    ids.push(*l);
                }
                ids
            },
            vec![super::COLOR],
            vec![super::COLOR_NAME.to_string()],
        );
        Self {
            base,
            shared_data,
            render_data,
        }
    }
}
