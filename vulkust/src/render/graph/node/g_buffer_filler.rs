use super::super::super::command::Buffer as CmdBuffer;
use super::super::super::config::Configurations;
use super::super::super::engine::Engine;
use super::super::super::framebuffer::Framebuffer;
use super::super::super::gapi::GraphicApiEngine;
use super::super::super::image::{AttachmentType, Format, View as ImageView};
use super::super::super::pipeline::{Pipeline, PipelineType};
use super::super::super::render_pass::RenderPass;
use super::super::super::sampler::Filter;
use super::super::super::sync::Semaphore;
use super::super::super::texture::{Manager as TextureManager, Texture};
use super::{Node, LinkId};
use std::sync::{Arc, Mutex, RwLock};

const LINKS_NAMES: [&'static str; 4] = [
    super::POSITION_NAME,
    super::NORMAL_NAME,
    super::ALBEDO_NAME,
    super::DEPTH_NAME,
];

const LINKS_IDS: [LinkId; 4] = [
    super::POSITION,
    super::NORMAL,
    super::ALBEDO,
    super::DEPTH,
];

#[cfg_attr(debug_mode, derive(Debug))]
struct KernelData {
    sec_cmd: CmdBuffer,
}

/// This struct is gonna be created for each instance
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
#[derive(Clone)]
struct SharedData {
    textures: Vec<Arc<RwLock<Texture>>>,
    render_pass: Arc<RenderPass>,
    framebuffer: Arc<Framebuffer>,
    pipeline: Arc<Pipeline>,
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct GBufferFiller {
    shared_data: SharedData,
    frames_data: Vec<FrameData>,
}

impl GBufferFiller {
    pub(super) fn new(eng: &Engine) -> Self {
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
        let frames_count = geng.get_frames_count();
        let mut frames_data = Vec::with_capacity(frames_count);
        for _ in 0..frames_count {
            frames_data.push(FrameData::new(&geng));
        }
        Self {
            shared_data,
            frames_data,
        }
    }

    pub(super) fn begin_secondary(&self, cmd: &mut CmdBuffer) {
        cmd.begin_secondary(&self.framebuffer);
        cmd.bind_pipeline(&self.pipeline);
    }

    pub(super) fn begin_primary(&self, cmd: &mut CmdBuffer) {
        self.framebuffer.begin(cmd);
    }

    pub(super) fn get_textures(&self) -> &Vec<Arc<RwLock<Texture>>> {
        return &self.textures;
    }

    pub(super) fn get_normal_texture(&self) -> &Arc<RwLock<Texture>> {
        return &self.textures[1];
    }

    pub(super) fn get_position_texture(&self) -> &Arc<RwLock<Texture>> {
        return &self.textures[0];
    }

    pub(super) fn get_depth_texture(&self) -> &Arc<RwLock<Texture>> {
        return &self.textures[3];
    }

    pub(super) fn get_framebuffer(&self) -> &Framebuffer {
        return &self.framebuffer;
    }
}

unsafe impl Send for GBufferFiller {}

unsafe impl Sync for GBufferFiller {}

impl Node for GBufferFiller {
    fn get_name(&self) -> &str {
        "G-Buffer-Filler"
    }

    fn get_input_links_names(&self) -> &[&str] {
        &[]
    }

    fn get_input_links_ids(&self) -> &[LinkId] { 
        &[]
    }

    fn get_input_link_index_by_name(&self, _: &str) -> Option<usize> {
        vxunexpected!();
    }

    fn get_input_link_index_by_id(&self, _: LinkId) -> Option<usize> {
        vxunexpected!();
    }

    fn get_output_links_names(&self) -> &[&str] {
        &LINKS_NAMES
    }

    fn get_output_links_ids(&self) -> &[LinkId] {
        &LINKS_IDS
    }

    fn get_output_link_index_by_name(&self, name: &str) -> Option<usize> {
        let mut i = 0;
        for n in &LINKS_NAMES {
            if *n == name {
                return Some(i);
            }
            i += 1;
        } 
        None
    }

    fn get_output_link_index_by_id(&self, id: LinkId) -> Option<usize> {
        let mut i = 0;
        for l in &LINKS_IDS {
            if *l == id {
                return Some(i);
            }
            i += 1;
        } 
        None
    }

    fn get_link_consumers(&self, index: usize) -> &[Weak<RwLock<Node>>];
    fn get_all_consumers(&self) -> &[Vec<Weak<RwLock<Node>>>];
    fn get_link_provider(&self, usize) -> &Arc<RwLock<Node>>;
    fn get_all_providers(&self) -> &[Arc<RwLock<Node>>];
    fn register_consumer_for_link(&mut self, usize, Weak<RwLock<Node>>);
    fn register_provider_for_link(&mut self, usize, Arc<RwLock<Node>>);
    fn create_new(&self) -> Arc<RwLock<Node>>;
    fn get_output_texture(&self, usize) -> Arc<RwLock<Texture>>;
    fn record(&self, kernel_index: usize, &Scene, &Engine);
    fn submit(&self, &Engine);
}
