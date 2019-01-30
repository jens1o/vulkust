/// Node is reponsible to record command buffers.
/// It gathers all the needed data like pipeline, descriptor-sets, ... .
/// Its each node has input and output links.
/// There are some predifined ids for links, in addition users of engine can specify their ids
/// Node must bring a structure for its output link, and must accept a trait as its dependancy
/// 

pub mod shadow_mapper;
pub mod shadow_accumulator;
pub mod g_buffer_filler;
pub mod ssao;
pub mod ssr;