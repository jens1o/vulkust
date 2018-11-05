use super::super::core::types::Real;
use super::command::Buffer as CmdBuffer;
use super::config::Configurations;
use super::framebuffer::Framebuffer;
use super::gapi::GraphicApiEngine;
use super::descriptor::Set as DescriptorSet;
use super::light::ShadowAccumulatorDirectionalUniform;
use super::image::{AttachmentType, Format as ImageFormat, View as ImageView};
use super::pipeline::{Pipeline, PipelineType};
use super::texture::Manager as TextureManager;
use super::render_pass::RenderPass;
use super::resolver::Resolver;
use std::sync::Arc;
use std::mem::size_of;

use math;

const SHADOW_MAP_FMT: ImageFormat = ImageFormat::DepthFloat;
const SHADOW_ACCUMULATOR_STRENGTH_FMT: ImageFormat = ImageFormat::Float;
const SHADOW_ACCUMULATOR_FLAGBITS_FMT: ImageFormat = ImageFormat::FlagBits64;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Shadower {
    //---------------------------------------
    shadow_map_buffers: Vec<Arc<ImageView>>,
    shadow_map_render_pass: Arc<RenderPass>,
    shadow_map_framebuffers: Vec<Arc<Framebuffer>>,
    shadow_map_pipeline: Arc<Pipeline>,
    shadow_map_descriptor_set: DescriptorSet,
    //---------------------------------------
    shadow_accumulator_strength_buffer: Arc<ImageView>,
    shadow_accumulator_flagbits_buffer: Arc<ImageView>,
    shadow_accumulator_directional_pipeline: Arc<Pipeline>,
    shadow_accumulator_directional_descriptor_set: DescriptorSet,
    shadow_accumulator_render_pass: Arc<RenderPass>,
    shadow_accumulator_framebuffer: Arc<Framebuffer>,
    clear_shadow_accumulator_render_pass: Arc<RenderPass>,
    clear_shadow_accumulator_framebuffer: Arc<Framebuffer>,
}

impl Shadower {
    pub(super) fn new(geng: &GraphicApiEngine, resolver: &Resolver, conf: &Configurations,
        texture_manager: &mut TextureManager) -> Self {
        let dev = geng.get_device();
        let memmgr = geng.get_memory_manager();
        let mut shadow_map_buffers = Vec::with_capacity(conf.get_max_shadow_maps_count() as usize);
        let mut shadow_map_textures = Vec::with_capacity(conf.get_max_shadow_maps_count() as usize);
        let sampler = geng.get_linear_repeat_sampler();
        for _ in 0..conf.get_max_shadow_maps_count() {
            let buf = Arc::new(ImageView::new_attachment(
                memmgr,
                SHADOW_MAP_FMT,
                1,
                AttachmentType::DepthShadowBuffer,
                conf.get_shadow_map_aspect(),
                conf.get_shadow_map_aspect(),
            ));
            shadow_map_textures.push(texture_manager.create_2d_with_view_sampler(buf.clone(), sampler.clone()));
            shadow_map_buffers.push(buf);
        }
        let shadow_accumulator_strength_buffer = Arc::new(ImageView::new_surface_attachment(
            dev.clone(),
            memmgr,
            SHADOW_ACCUMULATOR_STRENGTH_FMT,
            1,
            AttachmentType::ColorDisplay,
        ));
        let shadow_accumulator_flagbits_buffer = Arc::new(ImageView::new_surface_attachment(
            dev.clone(),
            memmgr,
            SHADOW_ACCUMULATOR_FLAGBITS_FMT,
            1,
            AttachmentType::ColorDisplay,
        ));
        let shadow_accumulator_buffers = vec![shadow_accumulator_strength_buffer.clone(), shadow_accumulator_flagbits_buffer.clone()];
        let clear_shadow_accumulator_render_pass = Arc::new(RenderPass::new(
            shadow_accumulator_buffers.clone(),
            true,
            false,
        ));
        let shadow_accumulator_render_pass = Arc::new(RenderPass::new(
            shadow_accumulator_buffers.clone(),
            false,
            true,
        ));
        let shadow_map_render_pass = Arc::new(RenderPass::new(
            vec![shadow_map_buffers[0].clone()],
            true,
            true,
        ));
        let clear_shadow_accumulator_framebuffer = Arc::new(Framebuffer::new(
            shadow_accumulator_buffers.clone(),
            shadow_accumulator_render_pass.clone(),
        ));
        let shadow_accumulator_framebuffer = Arc::new(Framebuffer::new(
            shadow_accumulator_buffers.clone(),
            clear_shadow_accumulator_render_pass.clone(),
        ));
        let mut shadow_map_framebuffers = Vec::new();
        for v in &shadow_map_buffers {
            shadow_map_framebuffers.push(Arc::new(Framebuffer::new(
                vec![v.clone()],
                shadow_map_render_pass.clone(),
            )));
        }
        let (shadow_mapper_uniform_buffer, shadow_accumulator_directional_uniform_buffer) = {
            let mut bufmgr = vxresult!(geng.get_buffer_manager().write());
            (bufmgr.create_dynamic_buffer(size_of::<ShadowMapperUniform>() as isize),
            bufmgr.create_dynamic_buffer(size_of::<ShadowAccumulatorDirectionalUniform>() as isize))
        };
        let (shadow_map_descriptor_set, shadow_accumulator_directional_descriptor_set) = {
            let mut desmgr = vxresult!(geng.get_descriptor_manager().write());
            let restex = resolver.get_output_textures();
            (desmgr.create_buffer_only_set(&shadow_mapper_uniform_buffer), 
            desmgr.create_shadow_accumulator_directional_set(
                &shadow_accumulator_directional_uniform_buffer,
                vec![vec![restex[0].clone()], vec![restex[1].clone()], shadow_map_textures]))
        };
        shadow_map_framebuffers.shrink_to_fit();
        let mut pipmgr = vxresult!(geng.get_pipeline_manager().write());
        let shadow_map_pipeline = 
            pipmgr.create(shadow_map_render_pass.clone(), PipelineType::ShadowMapper);
        let shadow_accumulator_directional_pipeline = pipmgr.create(shadow_accumulator_render_pass.clone(), PipelineType::ShadowAccumulatorDirectional);
        Self {
            shadow_map_buffers,
            shadow_map_render_pass,
            shadow_map_framebuffers,
            shadow_map_pipeline,
            shadow_map_descriptor_set,
            //---------------------------------------
            shadow_accumulator_strength_buffer,
            shadow_accumulator_flagbits_buffer,
            shadow_accumulator_directional_pipeline,
            shadow_accumulator_directional_descriptor_set,
            shadow_accumulator_render_pass,
            shadow_accumulator_framebuffer,
            clear_shadow_accumulator_render_pass,
            clear_shadow_accumulator_framebuffer,
        }
    }

    pub(super) fn begin_secondary_shadow_mappers(&self, cmds: &mut [CmdBuffer]) {
        let cmds_len = cmds.len();
        for i in 0..cmds_len {
            cmds[i].begin_secondary(&self.shadow_map_framebuffers[i]);
            cmds[i].bind_pipeline(&self.shadow_map_pipeline);
        }
    }

    pub(super) fn begin_shadow_map_primary(&self, cmd: &mut CmdBuffer, map_index: usize) {
        self.shadow_map_framebuffers[map_index].begin(cmd);
    }

    pub(crate) fn get_shadow_map_descriptor_set(&self) -> &DescriptorSet {
        return &self.shadow_map_descriptor_set;
    }

    // pub(super) fn begin_accumulator_primary(&self, cmd: &mut CmdBuffer) {
    //     self.shadow_map_framebuffers[map_index].begin(cmd);
    // }

    // do thread shadow gathering
    // do main thread shadow accumulating
}

unsafe impl Send for Shadower {}
unsafe impl Sync for Shadower {}

#[repr(C)]
struct ShadowMapperUniform {
    mvp: math::Matrix4<Real>
}