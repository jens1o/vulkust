use super::super::super::super::core::types::Real;
use super::super::super::buffer::{Buffer, Dynamic as DynamicBuffer, Manager as BufferManager};
use super::super::super::command::Buffer as CmdBuffer;
use super::super::super::descriptor::Set as DescriptorSet;
use super::super::super::engine::Engine;
use super::super::super::framebuffer::Framebuffer;
use super::super::super::gapi::GraphicApiEngine;
use super::super::super::image::{AttachmentType, Format, View as ImageView};
use super::super::super::pipeline::{Pipeline, PipelineType};
use super::super::super::render_pass::RenderPass;
use super::super::super::sampler::Filter as SamplerFilter;
use super::super::super::sync::Semaphore;
use super::super::super::texture::Texture;
use super::{Base, Node};
use std::sync::{Arc, Mutex, RwLock, Weak};

use cgmath;
use cgmath::SquareMatrix;

#[cfg_attr(debug_mode, derive(Debug))]
struct KernelData {
    sec_cmd: CmdBuffer,
    objects_count: usize,
    materials_data: Vec<(Weak<RwLock<Buffer>>, Weak<DescriptorSet>)>,
}

impl KernelData {
    fn new(geng: &GraphicApiEngine) -> Self {
        Self {
            sec_cmd: geng.create_secondary_command_buffer_from_main_graphic_pool(),
            objects_count: 0,
            materials_data: Vec::new(),
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct FrameData {
    pri_cmd: CmdBuffer,
    semaphore: Arc<Semaphore>,
    kernels_data: Vec<Mutex<KernelData>>,
}

impl FrameData {
    fn new(geng: &GraphicApiEngine) -> Self {
        let pri_cmd = geng.create_primary_command_buffer_from_main_graphic_pool();
        let semaphore = Arc::new(geng.create_semaphore());
        let kernels_count = num_cpus::get();
        let mut kernels_data = Vec::with_capacity(kernels_count);
        for _ in 0..kernels_count {
            kernels_data.push(Mutex::new(KernelData::new(geng)));
        }
        Self {
            pri_cmd,
            semaphore,
            kernels_data,
        }
    }
}

#[repr(C)]
#[cfg_attr(debug_mode, derive(Debug))]
struct Uniform {
    mvp: cgmath::Matrix4<Real>,
}

impl Uniform {
    pub fn new() -> Self {
        Self {
            mvp: cgmath::Matrix4::identity(),
        }
    }
}

/// This struct is gonna be created for each instance
#[cfg_attr(debug_mode, derive(Debug))]
struct RenderData {
    frames_data: Vec<FrameData>,
    kernels_uniform_buffers: Vec<Mutex<Vec<DynamicBuffer>>>,
}

impl RenderData {
    fn new(geng: &GraphicApiEngine) -> Self {
        let frames_count = geng.get_frames_count();
        let mut frames_data = Vec::with_capacity(frames_count);
        for _ in 0..frames_count {
            frames_data.push(FrameData::new(geng));
        }
        let kernels_count = num_cpus::get();
        let mut kernels_uniform_buffers = Vec::with_capacity(kernels_count);
        for _ in 0..kernels_count {
            kernels_uniform_buffers.push(Mutex::new(Vec::new()));
        }
        Self {
            frames_data,
            kernels_uniform_buffers,
        }
    }
}

/// This struct can be shared between instances
#[cfg_attr(debug_mode, derive(Debug))]
#[derive(Clone)]
struct SharedData {
    buffer_manager: Arc<RwLock<BufferManager>>,
    texture: Arc<RwLock<Texture>>,
    render_pass: Arc<RenderPass>,
    framebuffer: Arc<Framebuffer>,
    pipeline: Arc<Pipeline>,
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct ShadowMapper {
    base: Base,
    shared_data: SharedData,
    render_data: RenderData,
}

impl ShadowMapper {
    /// I can guess this is better to have totally new instance of this structre for each light's frustum.
    /// In this way we definitly can have better parallel code.
    pub fn new(eng: &Engine, width: usize, height: usize) -> Self {
        let geng = eng.get_gapi_engine();
        let geng = vxresult!(geng.read());
        let memmgr = geng.get_memory_manager();
        let buffers = vec![Arc::new(ImageView::new_attachment(
            memmgr,
            Format::DepthFloat32,
            AttachmentType::Depth,
            width as u32,
            height as u32,
        ))];
        let buffer_manager = geng.get_buffer_manager().clone();
        let texture = vxresult!(eng.get_asset_manager().get_texture_manager().write())
            .create_2d_with_view_sampler(
                buffers[0].clone(),
                vxresult!(geng.get_sampler_manager().write()).load(SamplerFilter::Linear),
            );
        let render_pass = Arc::new(RenderPass::new(buffers.clone(), true, true));
        let framebuffer = Arc::new(Framebuffer::new(buffers, render_pass.clone()));
        let pipeline = vxresult!(geng.get_pipeline_manager().write()).create(
            render_pass.clone(),
            PipelineType::SSAO,
            eng.get_config(),
        );
        let base = Base::new(
            super::SHADOW_MAPPER_NODE,
            "shadow-mapper".to_string(),
            Vec::new(),
            Vec::new(),
            vec![super::SINGLE_OUTPUT_NAME_LINK.to_string()],
            vec![super::SINGLE_OUTPUT_LINK],
        );
        let shared_data = SharedData {
            buffer_manager,
            pipeline,
            render_pass,
            framebuffer,
            texture,
        };
        let render_data = RenderData::new(&*geng);
        Self {
            base,
            shared_data,
            render_data,
        }
    }
}

impl Node for ShadowMapper {
    fn get_base(&self) -> &Base {
        &self.base
    }

    fn get_mut_base(&mut self) -> &mut Base {
        &mut self.base
    }

    fn create_new(&self, geng: &GraphicApiEngine) -> Arc<RwLock<Node>> {
        Arc::new(RwLock::new(Self {
            base: self.base.create_new(),
            shared_data: self.shared_data.clone(),
            render_data: RenderData::new(geng),
        }))
    }

    fn get_output_texture(&self, index: usize) -> &Arc<RwLock<Texture>> {
        #[cfg(debug_mode)]
        {
            if index != 0 {
                vxlogf!("Index out of range.");
            }
        }
        &self.shared_data.texture
    }
}
