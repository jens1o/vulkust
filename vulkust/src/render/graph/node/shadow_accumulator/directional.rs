use super::super::super::super::super::core::types::Real;
use super::super::super::super::config::MAX_DIRECTIONAL_CASCADES_MATRIX_COUNT;
use super::super::super::super::engine::Engine;
use super::super::super::super::gapi::GraphicApiEngine;
use super::super::super::super::image::Format;
use super::super::super::super::pipeline::PipelineType;
use super::super::super::super::texture::Texture;
use super::super::effect::{Base as EffectBase, BufferInfo, InputInfo};
use super::super::{Base as NodeBase, Node};
use cgmath;
use cgmath::SquareMatrix;
use std::sync::{Arc, RwLock};

pub const INPUT_INFOS: [InputInfo; 8] = [
    InputInfo {
        id: super::super::POSITION_LINK,
        name: super::super::POSITION_NAME_LINK,
    },
    InputInfo {
        id: super::super::NORMAL_LINK,
        name: super::super::NORMAL_NAME_LINK,
    },
    InputInfo {
        id: super::super::SHADOW_MAP_0_LINK,
        name: super::super::SHADOW_MAP_0_NAME_LINK,
    },
    InputInfo {
        id: super::super::SHADOW_MAP_1_LINK,
        name: super::super::SHADOW_MAP_1_NAME_LINK,
    },
    InputInfo {
        id: super::super::SHADOW_MAP_2_LINK,
        name: super::super::SHADOW_MAP_2_NAME_LINK,
    },
    InputInfo {
        id: super::super::SHADOW_MAP_3_LINK,
        name: super::super::SHADOW_MAP_3_NAME_LINK,
    },
    InputInfo {
        id: super::super::SHADOW_MAP_4_LINK,
        name: super::super::SHADOW_MAP_4_NAME_LINK,
    },
    InputInfo {
        id: super::super::SHADOW_MAP_5_LINK,
        name: super::super::SHADOW_MAP_5_NAME_LINK,
    },
];

#[repr(C)]
#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct Uniform {
    pub view_projection_biases:
        [cgmath::Matrix4<Real>; MAX_DIRECTIONAL_CASCADES_MATRIX_COUNT as usize],
    pub direction_strength: cgmath::Vector4<Real>,
    pub cascades_count: u32,
    pub light_index: u32,
}

impl Uniform {
    pub fn new() -> Self {
        Self {
            view_projection_biases: [cgmath::Matrix4::identity();
                MAX_DIRECTIONAL_CASCADES_MATRIX_COUNT as usize],
            direction_strength: cgmath::Vector4::new(0.0, 0.0, -1.0, 1.0),
            cascades_count: 0u32,
            light_index: 0u32,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Directional {
    base: EffectBase<Uniform>,
}

impl Directional {
    pub fn new(eng: &Engine, width: usize, height: usize) -> Self {
        let base = EffectBase::new_with_buffer_info(
            eng,
            &[BufferInfo {
                width: width,
                height: height,
                format: Format::FlagBits8,
                id: super::super::SINGLE_OUTPUT_LINK,
                name: super::super::SINGLE_OUTPUT_NAME_LINK,
            }],
            Uniform::new(),
            PipelineType::ShadowAccumulatorDirectional,
            super::super::SHADOW_ACCUMULATOR_DIRECTIONAL_NODE,
            "shadow-accumulator for directional lights",
            &INPUT_INFOS,
        );
        Self { base }
    }
}

impl Node for Directional {
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
