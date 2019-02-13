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
    DeferredPBR,
    /// For providing g-buffer for DeferredPBR
    GBufferFiller,
    /// For forward rendering of a single light with shadow map.
    /// The result of this can be accumulated for more call of other forward based PBRs
    ForwardSingleLightShadowPBR,
    /// This is like ForwardSingleLightShadowPBR but instead of a light and
    /// its corresponding shadow, this only has array of lights
    ForwardPBR,
    /// This is like ForwardSingleLightShadowPBR but instead of a light and
    /// its corresponding shadow, this only has an environment, and ambient light
    ForwardEnvironmentalPBR,
    ShadowAccumulatorDirectional,
    ShadowAccumulatorPoint,
    ShadowAccumulatorCone,
    ShadowMapper,
    SSAO,
    SSR,
    Unlit,
}
