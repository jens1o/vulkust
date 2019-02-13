#[cfg(blank_gapi)]
pub(crate) use super::super::blank_gapi::pipeline::*;
#[cfg(directx12_api)]
pub(crate) use super::super::d3d12::pipeline::*;
#[cfg(metal_api)]
pub(crate) use super::super::metal::pileline::*;
#[cfg(vulkan_api)]
pub(crate) use super::super::vulkan::pipeline::*;

#[repr(u8)]
#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum PipelineType {
    /// For deferred base PBR
    DeferredPbr,
    /// For providing g-buffer for DeferredPbr
    GBufferFiller,
    /// For forward-pbr rendering of one light with shadow map.
    /// The result of this can be accumulated for more call of other forward based PBRs
    ForwardPbrOneLightShadow,
    /// This is like ForwardPbrOneLightShadow but in addition to
    /// one light and its corresponding shadow-map, this has
    /// array of lights (do not cast shadow)
    /// and an environment
    /// and ambient light
    /// and a single
    ForwardPbr,
    GamaCorrectness,
    ShadowAccumulatorDirectional,
    ShadowAccumulatorPoint,
    ShadowAccumulatorCone,
    ShadowMapper,
    LightBlooming,
    SSAO,
    SSR,
    Unlit,
}
