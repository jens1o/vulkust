use super::super::super::core::storage::Storage;
use super::super::command::Buffer as CmdBuffer;
use super::super::config::Configurations;
use super::super::framebuffer::Framebuffer;
use super::super::gapi::GraphicApiEngine;
use super::super::image::{AttachmentType, Format, View as ImageView};
use super::super::pipeline::{Pipeline, PipelineType};
use super::super::render_pass::RenderPass;
use super::super::texture::{Manager as TextureManager, Texture};
use super::deferred_pbr::DeferredPBR;
use super::directional_shadow_accumulator::DirectionalShadowAccumulator;
use super::g_buffer_filler::GBufferFiller;
use super::shadow_mapper::ShadowMapper;
use super::Pass;
use std::sync::Arc;

/// A manager structure for passes
///
/// On its initialization it tries to initialize all the predefined passes.
/// User can add a customized pass through ```add```
/// It should handle dependancies and synchronizations

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Manager {
    // Predefined passes
    deferred: Arc<DeferredPBR>,
    directional_shadow_accumulator: Arc<DirectionalShadowAccumulator>,
    gbuffer: Arc<GBufferFiller>,
    transparent_pbr: Arc<TransparentPBR>,
    shadow_mapper: Arc<ShadowMapper>,
    ssao: Arc<SSAO>,
    unlit: Arc<Unlit>,
    // storage
    storage: Storage<Pass>,
}

impl Manager {
    pub fn new(
        eng: &GraphicApiEngine,
        texmgr: &mut TextureManager,
        config: &Configurations,
    ) -> Self {
        let transparent = Arc::new();
        Self {
            storage: Storage::new(),
        }
    }
}
