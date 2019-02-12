use super::super::core::config::Configurations;
use super::super::system::os::application::Application as OsApp;
use super::buffer::Manager as BufferManager;
use super::command::{Buffer as CmdBuffer, Pool as CmdPool, Type as CmdPoolType};
use super::descriptor::Manager as DescriptorManager;
use super::device::Logical as LogicalDevice;
use super::device::Physical as PhysicalDevice;
use super::framebuffer::Framebuffer;
use super::image::View as ImageView;
use super::instance::Instance;
use super::memory::Manager as MemoryManager;
use super::pipeline::Manager as PipelineManager;
use super::render_pass::RenderPass;
use super::sampler::Manager as SamplerManager;
use super::surface::Surface;
use super::swapchain::{NextImageResult, Swapchain};
use super::sync::Fence;
use super::sync::Semaphore;
use ash::version::DeviceV1_0;
use ash::vk;
use num_cpus;
use std::sync::{Arc, Mutex, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
struct KernelData {
    graphic_cmd_pool: Arc<CmdPool>,
}

impl KernelData {
    fn new(logical_device: Arc<LogicalDevice>) -> Self {
        let graphic_cmd_pool = Arc::new(CmdPool::new(
            logical_device,
            CmdPoolType::Graphic,
            vk::CommandPoolCreateFlags::empty(),
        ));
        Self { graphic_cmd_pool }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct FrameData {
    present_semaphore: Arc<Semaphore>,
    data_semaphore: Arc<Semaphore>,
    second_data_semaphore: Arc<Semaphore>,
    render_semaphore: Arc<Semaphore>,
    data_primary_cmd: Arc<Mutex<CmdBuffer>>,
    second_data_primary_cmd: Arc<Mutex<CmdBuffer>>,
    wait_fence: Arc<Fence>,
    framebuffer: Arc<Framebuffer>,
    clear_framebuffer: Arc<Framebuffer>,
}

impl FrameData {
    fn new(
        logical_device: Arc<LogicalDevice>,
        graphic_cmd_pool: Arc<CmdPool>,
        render_pass: Arc<RenderPass>,
        clear_render_pass: Arc<RenderPass>,
        swapchain_view: Arc<ImageView>,
    ) -> Self {
        let present_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        let data_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        let second_data_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        let render_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        let data_primary_cmd =
            Arc::new(Mutex::new(CmdBuffer::new_primary(graphic_cmd_pool.clone())));
        let second_data_primary_cmd =
            Arc::new(Mutex::new(CmdBuffer::new_primary(graphic_cmd_pool)));
        let wait_fence = Arc::new(Fence::new_signaled(logical_device));
        let framebuffer = Arc::new(Framebuffer::new(vec![swapchain_view.clone()], render_pass));
        let clear_framebuffer = Arc::new(Framebuffer::new(vec![swapchain_view], clear_render_pass));
        Self {
            present_semaphore,
            data_semaphore,
            second_data_semaphore,
            render_semaphore,
            data_primary_cmd,
            second_data_primary_cmd,
            wait_fence,
            framebuffer,
            clear_framebuffer,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Engine {
    os_app: Arc<RwLock<OsApp>>,
    instance: Arc<Instance>,
    surface: Arc<Surface>,
    physical_device: Arc<PhysicalDevice>,
    logical_device: Arc<LogicalDevice>,
    swapchain: Arc<Swapchain>,
    graphic_cmd_pool: Arc<CmdPool>,
    frames_data: Vec<FrameData>,
    kernels_data: Vec<KernelData>,
    memory_manager: Arc<RwLock<MemoryManager>>,
    buffer_manager: Arc<RwLock<BufferManager>>,
    descriptor_manager: Arc<RwLock<DescriptorManager>>,
    pipeline_manager: Arc<RwLock<PipelineManager>>,
    sampler_manager: Arc<RwLock<SamplerManager>>,
    //---------------------------------------
    clear_render_pass: Arc<RenderPass>,
    render_pass: Arc<RenderPass>,
    //---------------------------------------
    current_frame_number: usize,
    frames_count: usize,
}

impl Engine {
    pub(crate) fn new(os_app: &Arc<RwLock<OsApp>>, conf: &Configurations) -> Self {
        let instance = Arc::new(Instance::new(conf.get_application_name()));
        let surface = Arc::new(Surface::new(&instance, os_app));
        let physical_device = Arc::new(PhysicalDevice::new(&surface));
        let logical_device = Arc::new(LogicalDevice::new(&physical_device, conf.get_render()));
        let swapchain = Arc::new(Swapchain::new(&logical_device));
        let frames_count = swapchain.get_image_views().len();
        let graphic_cmd_pool = Arc::new(CmdPool::new(
            logical_device.clone(),
            CmdPoolType::Graphic,
            vk::CommandPoolCreateFlags::empty(),
        ));
        let memory_manager = MemoryManager::new(&logical_device);
        let render_pass = Arc::new(RenderPass::new_with_swapchain(swapchain.clone(), false));
        let clear_render_pass = Arc::new(RenderPass::new_with_swapchain(swapchain.clone(), true));
        let mut frames_data = Vec::with_capacity(frames_count);
        for v in swapchain.get_image_views() {
            frames_data.push(FrameData::new(
                logical_device.clone(),
                graphic_cmd_pool.clone(),
                render_pass.clone(),
                clear_render_pass.clone(),
                v.clone(),
            ));
        }
        let kernels_count = num_cpus::get() as usize;
        let kernels_data = Vec::with_capacity(kernels_count);
        for _ in 0..kernels_count {
            kernels_data.push(KernelData::new(logical_device.clone()));
        }
        let sampler_manager = Arc::new(RwLock::new(SamplerManager::new(logical_device.clone())));
        let buffer_manager = Arc::new(RwLock::new(BufferManager::new(
            &memory_manager,
            &graphic_cmd_pool,
            32 * 1024 * 1024,
            32 * 1024 * 1024,
            32 * 1024 * 1024,
            frames_count as isize,
        )));
        let descriptor_manager = Arc::new(RwLock::new(DescriptorManager::new(
            &logical_device,
            conf.get_render(),
        )));
        let pipeline_manager = Arc::new(RwLock::new(PipelineManager::new(
            logical_device.clone(),
            descriptor_manager.clone(),
        )));
        let os_app = os_app.clone();
        Self {
            os_app,
            instance,
            surface,
            physical_device,
            logical_device,
            swapchain,
            graphic_cmd_pool,
            frames_data,
            kernels_data,
            memory_manager,
            descriptor_manager,
            pipeline_manager,
            buffer_manager,
            sampler_manager,
            render_pass,
            clear_render_pass,
            current_frame_number: 0,
            frames_count,
        }
    }

    pub(crate) fn start_rendering(&mut self) {
        self.current_frame_number = match self
            .swapchain
            .get_next_image_index(&self.frames_data[self.current_frame_number].present_semaphore)
        {
            NextImageResult::Next(c) => c,
            NextImageResult::NeedsRefresh => {
                vxlogf!("Problem with rereshing screen, engine needs refreshing.");
            }
        } as usize;
        self.frames_data[self.current_frame_number]
            .wait_fence
            .wait();
        self.frames_data[self.current_frame_number]
            .wait_fence
            .reset();

        self.clear_copy_data();

        self.secondary_data_preparing();
    }

    fn clear_copy_data(&self) {
        let frame_data = &self.frames_data[self.current_frame_number];
        let mut pcmd = vxresult!(frame_data.data_primary_cmd.lock());
        pcmd.begin();
        vxresult!(self.buffer_manager.write()).update(&mut *pcmd, self.current_frame_number);
        frame_data.clear_framebuffer.begin(&mut *pcmd);
        pcmd.end_render_pass();
        pcmd.end();
        self.submit(
            &frame_data.present_semaphore,
            &pcmd,
            &frame_data.data_semaphore,
        );
    }

    fn secondary_data_preparing(&self) {
        let frame_data = &self.frames_data[self.current_frame_number];
        let mut pcmd = vxresult!(frame_data.second_data_primary_cmd.lock());
        pcmd.begin();
        vxresult!(self.buffer_manager.write())
            .secondary_update(&mut *pcmd, self.current_frame_number);
        pcmd.end();
        self.submit(
            &frame_data.data_semaphore,
            &pcmd,
            &frame_data.second_data_semaphore,
        );
    }

    pub(crate) fn submit(&self, wait: &Semaphore, cmd: &CmdBuffer, signal: &Semaphore) {
        self.submit_with_fence(
            &[*wait.get_data()],
            &[*cmd.get_data()],
            &[*signal.get_data()],
            None,
        );
    }

    pub(crate) fn submit_with_fence(
        &self,
        waits: &[vk::Semaphore],
        cmds: &[vk::CommandBuffer],
        signals: &[vk::Semaphore],
        fence: Option<&Fence>,
    ) {
        let wait_stage_mask = vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
        let mut submit_info = vk::SubmitInfo::default();
        submit_info.p_wait_dst_stage_mask = &wait_stage_mask;
        submit_info.p_wait_semaphores = waits.as_ptr();
        submit_info.wait_semaphore_count = waits.len() as u32;
        submit_info.p_signal_semaphores = signals.as_ptr();
        submit_info.signal_semaphore_count = signals.len() as u32;
        submit_info.p_command_buffers = cmds.as_ptr();
        submit_info.command_buffer_count = cmds.len() as u32;
        let fence = if let Some(fence) = fence {
            *fence.get_data()
        } else {
            vk::Fence::null()
        };
        let vk_dev = self.logical_device.get_data();
        vxresult!(unsafe {
            vk_dev.queue_submit(
                self.logical_device.get_vk_graphic_queue(),
                &[submit_info],
                fence,
            )
        });
    }

    pub(crate) fn submit_multiple(
        &self,
        waits: &[&Semaphore],
        cmds: &[&CmdBuffer],
        signals: &[&Semaphore],
    ) {
        let mut waits_data = Vec::with_capacity(waits.len());
        let mut signals_data = Vec::with_capacity(signals.len());
        let mut cmds_data = Vec::with_capacity(cmds.len());
        for w in waits {
            waits_data.push(*w.get_data());
        }
        for s in signals {
            signals_data.push(*s.get_data());
        }
        for c in cmds {
            cmds_data.push(*c.get_data());
        }
        self.submit_with_fence(&waits_data, &cmds_data, &signals_data, None);
    }

    pub(crate) fn end(&self, wait: &Semaphore) {
        let frame_data = &self.frames_data[self.current_frame_number];
        self.submit_with_fence(
            &[*wait.get_data()],
            &[],
            &[*frame_data.render_semaphore.get_data()],
            Some(&frame_data.wait_fence),
        );

        let current_frame_number = self.current_frame_number as u32;

        let mut present_info = vk::PresentInfoKHR::default();
        present_info.swapchain_count = 1;
        present_info.p_swapchains = self.swapchain.get_data();
        present_info.p_image_indices = &current_frame_number;
        present_info.p_wait_semaphores = frame_data.render_semaphore.get_data();
        present_info.wait_semaphore_count = 1;
        vxresult!(unsafe {
            self.swapchain
                .get_loader()
                .queue_present(self.logical_device.get_vk_graphic_queue(), &present_info)
        });
    }

    // pub(crate) fn terminate(&mut self) {
    //     self.logical_device.wait_idle();
    // }

    pub(crate) fn create_texture_2d_with_pixels(
        &self,
        width: u32,
        height: u32,
        data: &[u8],
    ) -> Arc<ImageView> {
        Arc::new(ImageView::new_texture_2d_with_pixels(
            width,
            height,
            data,
            &self.buffer_manager,
        ))
    }

    pub(crate) fn create_texture_cube_with_pixels(
        &self,
        width: u32,
        height: u32,
        data: &[&[u8]; 6],
    ) -> Arc<ImageView> {
        Arc::new(ImageView::new_texture_cube_with_pixels(
            width,
            height,
            data,
            &self.buffer_manager,
        ))
    }

    pub(crate) fn create_command_pool(&self) -> Arc<CmdPool> {
        return Arc::new(CmdPool::new(
            self.logical_device.clone(),
            CmdPoolType::Graphic,
            vk::CommandPoolCreateFlags::empty(),
        ));
    }

    pub(crate) fn create_secondary_command_buffer(&self, kernel_index: usize) -> CmdBuffer {
        return CmdBuffer::new_secondary(self.kernels_data[kernel_index].graphic_cmd_pool.clone());
    }

    pub(crate) fn create_primary_command_buffer(&self, kernel_index: usize) -> CmdBuffer {
        return CmdBuffer::new_primary(self.kernels_data[kernel_index].graphic_cmd_pool.clone());
    }

    pub(crate) fn create_primary_command_buffer_from_main_graphic_pool(&self) -> CmdBuffer {
        return CmdBuffer::new_primary(self.graphic_cmd_pool.clone());
    }

    pub(crate) fn create_secondary_command_buffer_from_main_graphic_pool(&self) -> CmdBuffer {
        return CmdBuffer::new_secondary(self.graphic_cmd_pool.clone());
    }

    pub(crate) fn create_semaphore(&self) -> Semaphore {
        return Semaphore::new(self.logical_device.clone());
    }

    pub(crate) fn get_kernels_count(&self) -> usize {
        return self.kernels_data.len();
    }

    pub(crate) fn get_frames_count(&self) -> usize {
        return self.frames_data.len();
    }

    pub(crate) fn get_frame_number(&self) -> usize {
        return self.current_frame_number;
    }

    pub(crate) fn get_current_framebuffer(&self) -> &Arc<Framebuffer> {
        return &self.frames_data[self.current_frame_number].framebuffer;
    }

    pub(crate) fn get_starting_semaphore(&self) -> &Arc<Semaphore> {
        return &self.frames_data[self.current_frame_number].second_data_semaphore;
    }

    pub(crate) fn get_device(&self) -> &Arc<LogicalDevice> {
        return &self.logical_device;
    }

    pub(crate) fn get_memory_manager(&self) -> &Arc<RwLock<MemoryManager>> {
        return &self.memory_manager;
    }

    pub(crate) fn get_buffer_manager(&self) -> &Arc<RwLock<BufferManager>> {
        return &self.buffer_manager;
    }

    pub(crate) fn get_descriptor_manager(&self) -> &Arc<RwLock<DescriptorManager>> {
        return &self.descriptor_manager;
    }

    pub(crate) fn get_pipeline_manager(&self) -> &Arc<RwLock<PipelineManager>> {
        return &self.pipeline_manager;
    }

    pub(crate) fn get_sampler_manager(&self) -> &Arc<RwLock<SamplerManager>> {
        return &self.sampler_manager;
    }

    pub(crate) fn get_render_pass(&self) -> &Arc<RenderPass> {
        return &self.render_pass;
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.logical_device.wait_idle();
    }
}

unsafe impl Send for Engine {}

unsafe impl Sync for Engine {}
