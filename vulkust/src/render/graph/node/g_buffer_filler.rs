use super::super::super::super::core::types::Real;
use super::super::super::camera::Camera;
use super::super::super::command::Buffer as CmdBuffer;
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
use std::sync::{Arc, Mutex, RwLock};

const LINKS_NAMES: [&str; 4] = [
    super::POSITION_NAME_LINK,
    super::NORMAL_NAME_LINK,
    super::ALBEDO_NAME_LINK,
    super::DEPTH_NAME_LINK,
];

const LINKS_IDS: [LinkId; 4] = [
    super::POSITION_LINK,
    super::NORMAL_LINK,
    super::ALBEDO_LINK,
    super::DEPTH_LINK,
];

#[cfg_attr(debug_mode, derive(Debug))]
struct KernelData {
    sec_cmd: CmdBuffer,
}

#[cfg_attr(debug_mode, derive(Debug))]
struct FrameData {
    pri_cmd: CmdBuffer,
    semaphore: Arc<Semaphore>,
    kernels_data: Vec<Arc<Mutex<KernelData>>>,
}

impl FrameData {
    fn new(geng: &GraphicApiEngine) -> Self {
        let pri_cmd = geng.create_primary_command_buffer_from_main_graphic_pool();
        let semaphore = Arc::new(geng.create_semaphore());
        let kernels_count = geng.get_kernels_count();
        let mut kernels_data = Vec::with_capacity(kernels_count);
        for ki in 0..kernels_count {
            kernels_data.push(Arc::new(Mutex::new(KernelData {
                sec_cmd: geng.create_secondary_command_buffer(ki),
            })));
        }
        Self {
            pri_cmd,
            semaphore,
            kernels_data,
        }
    }
}

/// This struct is gonna be created for each instance
#[cfg_attr(debug_mode, derive(Debug))]
struct RenderData {
    frames_data: Vec<FrameData>,
}

impl RenderData {
    fn new(geng: &GraphicApiEngine) -> Self {
        let frames_count = geng.get_frames_count();
        let mut frames_data = Vec::with_capacity(frames_count);
        for _ in 0..frames_count {
            frames_data.push(FrameData::new(geng));
        }
        Self { frames_data }
    }
}

/// This struct is gonna be created for each instance
#[cfg_attr(debug_mode, derive(Debug))]
#[derive(Clone)]
struct SharedData {
    textures: Vec<Arc<RwLock<Texture>>>,
    render_pass: Arc<RenderPass>,
    framebuffer: Arc<Framebuffer>,
    pipeline: Arc<Pipeline>,
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct GBufferFiller {
    base: Base,
    shared_data: SharedData,
    render_data: RenderData,
}

impl GBufferFiller {
    pub fn new(eng: &Engine) -> Self {
        let geng = eng.get_gapi_engine();
        let geng = vxresult!(geng.read());
        let dev = geng.get_device();

        let memmgr = geng.get_memory_manager();
        let buffers = vec![
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(),
                memmgr,
                Format::RgbaFloat,
                AttachmentType::ColorGBuffer,
            )),
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(),
                memmgr,
                Format::RgbaFloat,
                AttachmentType::ColorGBuffer,
            )),
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(),
                memmgr,
                Format::RgbaFloat,
                AttachmentType::ColorGBuffer,
            )),
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(),
                memmgr,
                Format::DepthFloat,
                AttachmentType::DepthGBuffer,
            )),
        ];
        let sampler = vxresult!(geng.get_sampler_manager().write()).load(Filter::Nearest);
        let mut textures = Vec::with_capacity(buffers.len());
        {
            let mut texmgr = vxresult!(eng.get_asset_manager().get_texture_manager().write());
            for b in &buffers {
                textures.push(texmgr.create_2d_with_view_sampler(b.clone(), sampler.clone()));
            }
        }
        let render_pass = Arc::new(RenderPass::new(buffers.clone(), true, true));
        let framebuffer = Arc::new(Framebuffer::new(buffers, render_pass.clone()));
        let pipeline = vxresult!(geng.get_pipeline_manager().write()).create(
            render_pass.clone(),
            PipelineType::GBufferFiller,
            eng.get_config(),
        );
        let shared_data = SharedData {
            textures,
            render_pass,
            framebuffer,
            pipeline,
        };
        let render_data = RenderData::new(&*geng);
        let base = Base::new(
            super::G_BUFFER_FILLER_NODE,
            "g-buffer-filler".to_string(),
            Vec::new(),
            Vec::new(),
            {
                let mut names = Vec::with_capacity(LINKS_NAMES.len());
                for l in &LINKS_NAMES {
                    names.push(l.to_string());
                }
                names
            },
            {
                let mut ids = Vec::with_capacity(LINKS_IDS.len());
                for l in &LINKS_IDS {
                    ids.push(*l);
                }
                ids
            },
        );
        Self {
            shared_data,
            base,
            render_data,
        }
    }

    pub fn get_viewport(&mut self, _x: Real, _y: Real, _width: Real, _height: Real) {
        vxunimplemented!();
    }

    pub fn record(&mut self, _kernel_idex: usize, _camera: &Camera, _scene: &Scene) {
        vxunimplemented!();
    }

    pub fn submit(&self, _geng: &GraphicApiEngine) {
        vxunimplemented!();
    }

    // pub(super) fn begin_secondary(&self, cmd: &mut CmdBuffer) {
    //     cmd.begin_secondary(&self.framebuffer);
    //     cmd.bind_pipeline(&self.pipeline);
    // }

    // pub(super) fn begin_primary(&self, cmd: &mut CmdBuffer) {
    //     self.framebuffer.begin(cmd);
    // }
}

unsafe impl Send for GBufferFiller {}

unsafe impl Sync for GBufferFiller {}

impl Node for GBufferFiller {
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
        &self.shared_data.textures[index]
    }
}
