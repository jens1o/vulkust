#[cfg(blank_gapi)]
pub(crate) use super::super::blank_gapi::image::*;
#[cfg(directx12_api)]
pub use super::super::d3d12::image::*;
#[cfg(metal_api)]
pub use super::super::metal::image::*;
#[cfg(vulkan_api)]
pub use super::super::vulkan::image::*;

#[cfg_attr(debug_mode, derive(Debug))]
pub enum AttachmentType {
    Depth,
    DepthStencil,
    Display,
    Effect,
}

#[cfg_attr(debug_mode, derive(Debug))]
pub enum Format {
    RgbaFloat16,
    RgbaFloat32,
    RgbaByte,
    DepthFloat16,
    DepthFloat24,
    DepthFloat32,
    FloatUniform8,
    Float16,
    Float32,
    FlagBits8,
    FlagBits32,
    FlagBits64,
}

#[cfg_attr(debug_mode, derive(Debug))]
pub enum Layout {
    Uninitialized,
    DepthStencil,
    Display,
    ShaderReadOnly,
}

#[cfg_attr(debug_mode, derive(Debug))]
pub enum Usage {
    Color,
    DepthStencil,
}

#[cfg_attr(debug_mode, derive(Debug))]
pub enum ImageType {
    Cube,
    D2,
    D3,
}
