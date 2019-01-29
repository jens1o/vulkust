use super::super::core::debug::Debug;
use super::super::core::types::Id;
use super::pipeline::PipelineType;

pub mod node;
pub mod tree;

/// This module is made of `tree` and `node` module
///
/// Each tree made of several `Node`s that finally produce in result.
/// Tree itself can be a `Node` so you can combine several trees together.
///
/// Node is responsible of having pipeline, descriptors, ... .
/// It should be able to synchronize its dependancies and productions.
/// It create it should be able to create its needed commands,
/// handle multithreaded rendering,

pub trait Graph: Debug {
    type FrameData;
    fn get_pass_type(&self) -> Id;
    fn get_pipeline_type(&self) -> PipelineType;
}

#[repr(u64)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum PassType {
    DeferredPBR = 1,
    DirectionalShadowAccumulator = 2,
    GBufferFiller = 3,
    ShadowMapper = 4,
    SSAO = 5,
    TransparentPBR = 6,
    Unlit = 7,
}

/// User must start there pass ids from here
pub const USER_STARTING_TYPE_ID: Id = 1024;
