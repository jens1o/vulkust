use super::super::super::super::core::types::Real;
use super::super::super::engine::Engine;
use super::super::super::gapi::GraphicApiEngine;
use super::super::super::image::Format;
use super::super::super::pipeline::PipelineType;
use super::super::super::texture::Texture;
use super::effect::{Base as EffectBase, BufferInfo, InputInfo};
use super::{Base as NodeBase, Node};
use std::sync::{Arc, RwLock};

use cgmath;
use cgmath::InnerSpace;
use rand;
use rand::distributions::{Distribution as RandDis, Uniform as RandUni};

pub const INPUT_INFOS: [InputInfo; 3] = [
    InputInfo {
        id: super::POSITION_LINK,
        name: super::POSITION_NAME_LINK,
    },
    InputInfo {
        id: super::NORMAL_LINK,
        name: super::NORMAL_NAME_LINK,
    },
    InputInfo {
        id: super::DEPTH_LINK,
        name: super::DEPTH_NAME_LINK,
    },
];

const MAX_SSAO_SAMPLES_COUNT: usize = 128;

#[repr(C)]
#[derive(Clone)]
struct Uniform {
    sample_vectors: [cgmath::Vector4<Real>; MAX_SSAO_SAMPLES_COUNT],
}

#[cfg(debug_mode)]
impl std::fmt::Debug for Uniform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SSAO Uniform")
    }
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

#[cfg_attr(debug_mode, derive(Debug))]
pub struct SSAO {
    base: EffectBase<Uniform>,
}

impl SSAO {
    pub(crate) fn new(eng: &Engine, width: usize, height: usize) -> Self {
        let base = EffectBase::new_with_buffer_info(
            eng,
            &[BufferInfo {
                width,
                height,
                format: Format::FloatUniform8,
                id: super::SINGLE_OUTPUT_LINK,
                name: super::SINGLE_OUTPUT_NAME_LINK,
            }],
            Uniform::new(),
            PipelineType::SSAO,
            super::SSAO_NODE,
            "ssao",
            &INPUT_INFOS,
        );
        Self { base }
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
    fn get_base(&self) -> &NodeBase {
        self.base.get_base()
    }

    fn get_mut_base(&mut self) -> &mut NodeBase {
        self.base.get_mut_base()
    }

    fn create_new(&self, geng: &GraphicApiEngine) -> Arc<RwLock<Node>> {
        self.base.create_new(geng)
    }

    fn get_output_texture(&self, index: usize) -> &Arc<RwLock<Texture>> {
        self.base.get_output_texture(index)
    }
}
