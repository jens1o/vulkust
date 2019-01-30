/// Operation Type of a tree can be Graphic, Compute, Transpher, ...
/// A `Tree` itself can be a `Node`
/// A `Tree` can be feeded with several things like Model, Scene, Material, Mesh or other production of other tree
/// Tree will gather all the needed frame-data, render-data of it nodes

#[repr(u64)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum TreeType {
    DeferredPBR = 1,
    ForwardPBR = 2,
    Shadower = 3,
    Unlit = 4,
}

/// User must start their ids from here
pub const USER_STARTING_TYPE_ID: Id = 1024 * 1024;