use super::super::super::super::core::types::Real;
use super::super::super::buffer::{Buffer, Dynamic as DynamicBuffer};
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
use super::{Base, LinkId, Node};
use std::mem::size_of;
use std::sync::{Arc, RwLock, Weak};

use cgmath;
use cgmath::InnerSpace;
use rand;
use rand::distributions::{Distribution as RandDis, Uniform as RandUni};

const INPUT_LINKS_NAMES: [&str; 3] = [
    super::POSITION_NAME_LINK,
    super::NORMAL_NAME_LINK,
    super::DEPTH_NAME_LINK,
];

const INPUT_LINKS_IDS: [LinkId; 3] = [super::POSITION_LINK, super::NORMAL_LINK, super::DEPTH_LINK];

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
    kernels_data: Vec<KernelData>,
}

impl FrameData {
    fn new(geng: &GraphicApiEngine) -> Self {
        let pri_cmd = geng.create_primary_command_buffer_from_main_graphic_pool();
        let semaphore = Arc::new(geng.create_semaphore());
        let kernels_count = num_cpus::get();
        let mut kernels_data = Vec::with_capacity(kernels_count);
        for _ in 0..kernels_count {
            kernels_data.push(KernelData::new(geng));
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
    mvp: cgmath::Vector4<Real>,
}

impl Uniform {
    pub fn new() -> Self {
        let r1 = RandUni::from(-1f32..1f32);
        let r2 = RandUni::from(0f32..1f32);
        let mut rng = rand::thread_rng();
        let mut sample_vectors = [cgmath::Vector4::new(0.0, 0.0, 0.0, 0.0); MAX_SSAO_SAMPLES_COUNT];
        let mut sum_weight = 0.0;
        for i in 0..MAX_SSAO_SAMPLES_COUNT {
            let v = cgmath::Vector3::new(
                r1.sample(&mut rng),
                r1.sample(&mut rng),
                r2.sample(&mut rng),
            );
            let sv = &mut sample_vectors[i];
            sv.x = v.x;
            sv.y = v.y;
            sv.z = v.z;
            sv.w = 2.4 - v.magnitude();
            sum_weight += sv.w;
        }
        let coef = -1.0 / sum_weight;
        for i in 0..MAX_SSAO_SAMPLES_COUNT {
            sample_vectors[i].w *= coef;
        }
        Self { sample_vectors }
    }
}

/// This struct is gonna be created for each instance
#[cfg_attr(debug_mode, derive(Debug))]
struct RenderData {
    frames_data: Vec<FrameData>,
    input_textures: Vec<Option<Arc<RwLock<Texture>>>>,
}

impl RenderData {
    fn new(geng: &GraphicApiEngine) -> Self {
        let frames_count = geng.get_frames_count();
        let mut frames_data = Vec::with_capacity(frames_count);
        for _ in 0..frames_count {
            frames_data.push(FrameData::new(geng));
        }
        Self {
            frames_data,
            input_textures: Vec::new(),
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
    uniform_buffer: DynamicBuffer,
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct SSAO {
    base: Base,
    shared_data: SharedData,
    render_data: RenderData,
}

impl SSAO {
    pub(crate) fn new(eng: &Engine) -> Self {
        let geng = eng.get_gapi_engine();
        let geng = vxresult!(geng.read());
        let dev = geng.get_device();
        let memmgr = geng.get_memory_manager();
        let buffers = vec![Arc::new(ImageView::new_surface_attachment(
            dev.clone(),
            memmgr,
            Format::Float,
            AttachmentType::Effect,
        ))];
        let uniform = Uniform::new();
        let uniform_buffer = vxresult!(geng.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        for n in 0..geng.get_frames_count() {
            uniform_buffer.update(&uniform, n);
        }
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
            super::SSAO_NODE,
            "ssao".to_string(),
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
            vec![super::SINGLE_OUTPUT_NAME_LINK.to_string()],
            vec![super::SINGLE_OUTPUT_LINK],
        );
        let shared_data = SharedData {
            uniform_buffer,
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

    // pub(super) fn begin_secondary(&self, cmd: &mut CmdBuffer) {
    //     cmd.begin_secondary(&self.framebuffer);
    //     cmd.bind_pipeline(&self.pipeline);
    // }

    // pub(super) fn end_secondary(&self, cmd: &mut CmdBuffer, frame_number: usize) {
    //     let buffer = self.uniform_buffer.get_buffer(frame_number);
    //     let buffer = vxresult!(buffer.read());
    //     cmd.bind_ssao_ssao_descriptor(&*self.descriptor_set, &*buffer);
    //     cmd.render_ssao();
    //     cmd.end();
    // }

    // pub(super) fn record_primary(&self, pricmd: &mut CmdBuffer, seccmd: &CmdBuffer) {
    //     pricmd.begin();
    //     self.framebuffer.begin(pricmd);
    //     pricmd.exe_cmd(seccmd);
    //     pricmd.end_render_pass();
    //     pricmd.end();
    // }

    // pub(crate) fn update(&mut self, frame_number: usize) {
    //     self.uniform_buffer.update(&self.uniform, frame_number);
    // }

    // pub(crate) fn get_ambient_occlusion_texture(&self) -> &Arc<RwLock<Texture>> {
    //     return &self.textures[0];
    // }
}

impl Node for SSAO {
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

    fn register_provider_for_link(&mut self, index: usize, p: Arc<RwLock<Node>>, p_index: usize) {
        self.render_data.input_textures[index] =
            Some(vxresult!(p.read()).get_output_texture(p_index).clone());
        for fd in &mut self.render_data.frames_data {
            fd.input_textures_changed = true;
        }
        self.get_mut_base().register_provider_for_link(index, p);
    }
}
