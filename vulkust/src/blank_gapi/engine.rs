use super::super::render::config::Configurations;
use super::super::system::os::application::Application as OsApp;
use super::buffer::Manager as BufferManager;
use super::command::{Buffer as CmdBuffer, Pool as CmdPool};
use super::descriptor::Manager as DescriptorManager;
use super::device::Device;
use super::framebuffer::Framebuffer;
use super::image::View as ImageView;
use super::memory::Manager as MemoryManager;
use super::pipeline::Manager as PipelineManager;
use super::render_pass::RenderPass;
use super::sampler::Sampler;
use super::sync::Semaphore;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Engine {}

impl Engine {
    pub(crate) fn new(_os_app: &Arc<RwLock<OsApp>>, _conf: &Configurations) -> Self {
        vxunimplemented!();
    }

    pub(crate) fn get_device(&self) -> &Arc<Device> {
        vxunimplemented!();
    }

    pub(crate) fn get_linear_repeat_sampler(&self) -> &Arc<Sampler> {
        vxunimplemented!();
    }

    pub(crate) fn get_nearest_repeat_sampler(&self) -> &Arc<Sampler> {
        vxunimplemented!();
    }

    pub(crate) fn get_buffer_manager(&self) -> &Arc<RwLock<BufferManager>> {
        vxunimplemented!();
    }

    pub(crate) fn get_descriptor_manager(&self) -> &Arc<RwLock<DescriptorManager>> {
        vxunimplemented!();
    }

    pub(crate) fn get_pipeline_manager(&self) -> &Arc<RwLock<PipelineManager>> {
        vxunimplemented!();
    }

    pub(crate) fn get_memory_manager(&self) -> &Arc<RwLock<MemoryManager>> {
        vxunimplemented!();
    }

    pub(crate) fn get_render_pass(&self) -> &Arc<RenderPass> {
        // it should be removed in future, the corresponding render pass must move to deferred structure
        vxunexpected!();
    }

    pub(crate) fn create_command_pool(&self) -> Arc<CmdPool> {
        vxunimplemented!();
    }

    pub(crate) fn create_secondary_command_buffer(&self, _cmd_pool: Arc<CmdPool>) -> CmdBuffer {
        vxunimplemented!();
    }

    pub(crate) fn create_primary_command_buffer(&self, _cmd_pool: Arc<CmdPool>) -> CmdBuffer {
        vxunimplemented!();
    }

    pub(crate) fn create_primary_command_buffer_from_main_graphic_pool(&self) -> CmdBuffer {
        vxunimplemented!();
    }

    pub(crate) fn create_secondary_command_buffer_from_main_graphic_pool(&self) -> CmdBuffer {
        vxunimplemented!();
    }

    pub(crate) fn create_semaphore(&self) -> Semaphore {
        vxunimplemented!();
    }

    pub(crate) fn get_frames_count(&self) -> usize {
        vxunimplemented!();
    }

    pub(crate) fn get_frame_number(&self) -> usize {
        vxunimplemented!();
    }

    pub(crate) fn start_rendering(&mut self) {
        vxunimplemented!();
    }

    pub(crate) fn submit(&self, _wait: &Semaphore, _cmd: &CmdBuffer, _signal: &Semaphore) {
        vxunimplemented!();
    }

    pub(crate) fn submit_multiple(
        &self,
        _waits: &[&Semaphore],
        _cmds: &[&CmdBuffer],
        _signals: &[&Semaphore],
    ) {
        vxunimplemented!();
    }

    pub(crate) fn create_texture_2d_with_pixels(
        &self,
        _width: u32,
        _height: u32,
        _data: &[u8],
    ) -> Arc<ImageView> {
        vxunimplemented!();
    }

    pub(crate) fn get_current_framebuffer(&self) -> &Arc<Framebuffer> {
        // it should be removed in future, the corresponding framebuffer must move to deferred structure
        vxunexpected!();
    }

    pub(crate) fn end(&self, _wait: &Semaphore) {
        vxunimplemented!();
    }

    pub(crate) fn get_starting_semaphore(&self) -> &Arc<Semaphore> {
        vxunimplemented!();
    }
}
