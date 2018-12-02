use super::super::render::config::Configurations;
use super::super::render::pipeline::PipelineType;
use super::render_pass::RenderPass;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Pipeline {}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Manager {}

impl Manager {
    pub(crate) fn create(
        &mut self,
        render_pass: Arc<RenderPass>,
        pipeline_type: PipelineType,
        config: &Configurations,
    ) -> Arc<Pipeline> {
        vxunimplemented!();
    }
}
