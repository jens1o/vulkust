use super::super::super::super::core::debug::Debug as CoreDebug;
use super::super::super::buffer::Dynamic as DynamicBuffer;
use super::super::super::command::Buffer as CmdBuffer;
use super::super::super::descriptor::Set as DescriptorSet;
use super::super::super::engine::Engine;
use super::super::super::framebuffer::Framebuffer;
use super::super::super::gapi::GraphicApiEngine;
use super::super::super::image::{AttachmentType, Format, View as ImageView};
use super::super::super::pipeline::{Pipeline, PipelineType};
use super::super::super::render_pass::RenderPass;
use super::super::super::sampler::Filter;
use super::super::super::sync::Semaphore;
use super::super::super::texture::Texture;
use super::{Base as NodeBase, LinkId, Node, NodeId};
use std::mem::size_of;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
struct FrameData {
    pri_cmd: CmdBuffer,
    sec_cmd: CmdBuffer,
    semaphore: Arc<Semaphore>,
    descriptor_set: Option<Arc<DescriptorSet>>,
    /// Whenever this was true the descriptor set for that frame must be updated
    input_textures_changed: bool,
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
            descriptor_set: None,
            input_textures_changed: false,
        }
    }
}

/// This struct is gonna be created for each instance
#[cfg_attr(debug_mode, derive(Debug))]
struct RenderData<U>
where
    U: Sized + CoreDebug,
{
    frames_data: Vec<FrameData>,
    uniform: U,
    uniform_buffer: DynamicBuffer,
    input_textures: Vec<Option<Arc<RwLock<Texture>>>>,
}

impl<U> RenderData<U>
where
    U: 'static + Sized + CoreDebug + Clone,
{
    fn new(geng: &GraphicApiEngine, uniform: U) -> Self {
        let frames_count = geng.get_frames_count();
        let mut frames_data = Vec::with_capacity(frames_count);
        for _ in 0..frames_count {
            frames_data.push(FrameData::new(geng));
        }
        let uniform_buffer = vxresult!(geng.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<U>() as isize);
        Self {
            frames_data,
            uniform,
            uniform_buffer,
            input_textures: Vec::new(),
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
pub struct Base<U>
where
    U: 'static + Sized + CoreDebug + Clone,
{
    base: NodeBase,
    shared_data: SharedData,
    render_data: RenderData<U>,
}

pub struct BufferInfo {
    pub width: usize,
    pub height: usize,
    pub format: Format,
    pub id: LinkId,
    pub name: &'static str,
}

pub struct InputInfo {
    pub id: LinkId,
    pub name: &'static str,
}

impl<U> Base<U>
where
    U: 'static + Sized + CoreDebug + Clone,
{
    pub fn new_with_buffer_info(
        eng: &Engine,
        buffer_infos: &[BufferInfo],
        uniform: U,
        pipeline_type: PipelineType,
        node_id: NodeId,
        node_name: &str,
        input_infos: &[InputInfo],
    ) -> Self {
        let geng = eng.get_gapi_engine();
        let geng = vxresult!(geng.read());
        let dev = geng.get_device();
        let memmgr = geng.get_memory_manager();
        let mut buffers = Vec::with_capacity(buffer_infos.len());
        let mut textures = Vec::with_capacity(buffer_infos.len());
        for bf in buffer_infos {
            buffers.push(Arc::new(ImageView::new_attachment(
                memmgr,
                bf.format,
                AttachmentType::Effect,
                bf.width as u32,
                bf.height as u32,
            )));
        }
        {
            let sampler = vxresult!(geng.get_sampler_manager().write()).load(Filter::Nearest);
            let texture_manager = vxresult!(eng.get_asset_manager().get_texture_manager().write());
            for b in &buffers {
                textures
                    .push(texture_manager.create_2d_with_view_sampler(b.clone(), sampler.clone()));
            }
        }
        let render_pass = Arc::new(RenderPass::new(buffers.clone(), true, true));
        let framebuffer = Arc::new(Framebuffer::new(buffers, render_pass.clone()));
        let pipeline = vxresult!(geng.get_pipeline_manager().write()).create(
            render_pass.clone(),
            pipeline_type,
            eng.get_config(),
        );
        let base = NodeBase::new(
            node_id,
            node_name.to_string(),
            {
                let mut names = Vec::with_capacity(input_infos.len());
                for i in input_infos {
                    names.push(i.name.to_string());
                }
                names
            },
            {
                let mut ids = Vec::with_capacity(input_infos.len());
                for i in input_infos {
                    ids.push(i.id);
                }
                ids
            },
            {
                let mut names = Vec::with_capacity(buffer_infos.len());
                for bf in buffer_infos {
                    names.push(bf.name.to_string());
                }
                names
            },
            {
                let mut ids = Vec::with_capacity(buffer_infos.len());
                for bf in buffer_infos {
                    ids.push(bf.id);
                }
                ids
            },
        );
        let shared_data = SharedData {
            textures,
            framebuffer,
            render_pass,
            pipeline,
        };
        let render_data = RenderData::new(&geng, uniform);
        Self {
            base,
            shared_data,
            render_data,
        }
    }
}

impl<U> Node for Base<U>
where
    U: 'static + Sized + CoreDebug + Clone,
{
    fn get_base(&self) -> &NodeBase {
        &self.base
    }

    fn get_mut_base(&mut self) -> &mut NodeBase {
        &mut self.base
    }

    fn create_new(&self, geng: &GraphicApiEngine) -> Arc<RwLock<Node>> {
        Arc::new(RwLock::new(Self {
            base: self.base.create_new(),
            shared_data: self.shared_data.clone(),
            render_data: RenderData::new(geng, self.render_data.uniform.clone()),
        }))
    }

    fn get_output_texture(&self, index: usize) -> &Arc<RwLock<Texture>> {
        &self.shared_data.textures[index]
    }

    fn register_provider_for_link(&mut self, index: usize, p: Arc<RwLock<Node>>, p_index: usize) {
        self.render_data.input_textures[index] =
            Some(vxresult!(p.read()).get_output_texture(p_index).clone());
        for fd in &mut self.render_data.frames_data {
            fd.input_textures_changed = true;
        }
        self.get_mut_base().register_provider_for_link(index, p);
    }
}
