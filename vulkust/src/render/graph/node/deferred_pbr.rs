use super::super::super::super::core::types::Real;
use super::super::super::engine::Engine;
use super::super::super::gapi::GraphicApiEngine;
use super::super::super::image::Format;
use super::super::super::pipeline::PipelineType;
use super::super::super::texture::Texture;
use super::effect::{Base as EffectBase, BufferInfo, InputInfo};
use super::{Base as NodeBase, Node};
use cgmath;
use std::sync::{Arc, RwLock};

pub const INPUT_INFOS: [InputInfo; 6] = [
    InputInfo {
        id: super::POSITION_LINK,
        name: super::POSITION_NAME_LINK,
    },
    InputInfo {
        id: super::NORMAL_LINK,
        name: super::NORMAL_NAME_LINK,
    },
    InputInfo {
        id: super::ALBEDO_LINK,
        name: super::ALBEDO_NAME_LINK,
    },
    InputInfo {
        id: super::DEPTH_LINK,
        name: super::DEPTH_NAME_LINK,
    },
    InputInfo {
        id: super::OCCLUSION_LINK,
        name: super::OCCLUSION_NAME_LINK,
    },
    InputInfo {
        id: super::ACCUMULATED_SHADOWS_LINK,
        name: super::ACCUMULATED_SHADOWS_NAME_LINK,
    },
];

#[repr(C)]
#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
struct Uniform {
    pub pixel_step: cgmath::Vector4<Real>,
}

impl Uniform {
    pub fn new(window_width: Real, window_height: Real) -> Self {
        Self {
            pixel_step: cgmath::Vector4::new(1f32 / window_width, 1f32 / window_height, 0.0, 0.0),
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct DeferredPbr {
    base: EffectBase<Uniform>,
}

impl DeferredPbr {
    pub fn new(eng: &Engine, width: usize, height: usize) -> Self {
        let base = EffectBase::new_with_buffer_info(
            eng,
            &[BufferInfo {
                width: width,
                height: height,
                format: Format::RgbaFloat,
                id: super::SINGLE_OUTPUT_LINK,
                name: super::SINGLE_OUTPUT_NAME_LINK,
            }],
            Uniform::new(width as Real, height as Real),
            PipelineType::DeferredPbr,
            super::DEFERRED_PBR_NODE,
            "deferred-pbr",
            &INPUT_INFOS,
        );
        Self { base }
    }
}

impl Node for DeferredPbr {
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

    fn register_provider_for_link(&mut self, index: usize, p: Arc<RwLock<Node>>, p_index: usize) {
        self.base.register_provider_for_link(index, p, p_index)
    }
}
