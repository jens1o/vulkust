pub mod g_buffer_filler;
pub mod shadow_accumulator;
pub mod shadow_mapper;
pub mod ssao;
pub mod ssr;

use super::super::super::core::debug::Debug as CoreDebug;
use std::sync::{Arc, RwLock, Weak};

/// Node is reponsible to record command buffers.
/// It gathers all the needed data like pipeline, descriptor-sets, ... .
/// Each node has input and output links.
/// Node must provide render-data that is contain none-shared objects, like command buffer, semaphores, ...
/// There are some predifined ids for links, in addition users of engine can specify their ids
/// Node must bring a structure for its output link, and must accept a trait as its dependancy
/// Node will contain strong pointer for its providers and weak pointer for its consumers
/// A node must register itself a cunsumer it is, gather its data when provider provided its data.
/// A node must register its cunsumers and signal them whenever their needed data is gathered
/// So do not forget provider is responsible for signaling its registered consumers
/// and consumers are responsible for waiting on signal of provider for using data
/// Each node will get data of the light, scene, camera, model, ...
/// Always there is and must be a single provider for each dependancy
/// But there might be several or zero consumer for a data
/// This system can be used to push independant commands on separate queue

#[cfg_attr(debug_mode, derive(Debug))]
pub enum LinkId {
    Position,
    Normal,
    Tangent,
    Bitangent,
    Depth,
}

pub trait Node: CoreDebug {
    fn get_name(&self) -> &str;
    fn get_input_links_names(&self) -> &[&str];
    fn get_input_links_ids(&self) -> &[LinkId];
    fn get_input_link_index_by_name(&self, &str) -> Option<usize>;
    fn get_input_link_index_by_id(&self, LinkId) -> Option<usize>;
    fn get_output_links_names(&self) -> &[&str];
    fn get_output_links_ids(&self) -> &[LinkId];
    fn get_output_link_index_by_name(&self, &str) -> Option<usize>;
    fn get_output_link_index_by_id(&self, LinkId) -> Option<usize>;

    fn get_link_consumers(&self, usize) -> &[Weak<RwLock<Node>>];
    fn get_all_consumers(&self) -> &[Vec<Weak<RwLock<Node>>>];

    fn register_consumer_for_link_by_name(&mut self, name: &str, o: Weak<RwLock<Node>>) {
        self.register_consumer_for_link(vxunwrap!(self.get_output_link_index_by_name(name)), o);
    }

    fn register_consumer_for_link_by_id(&mut self, id: LinkId, o: Weak<RwLock<Node>>) {
        self.register_consumer_for_link(vxunwrap!(self.get_output_link_index_by_id(id)), o);
    }

    fn register_consumer_for_link(&mut self, usize, Weak<RwLock<Node>>);

    fn get_link_provider(&self, usize) -> &Arc<RwLock<Node>>;
    fn get_all_providers(&self) -> &[Arc<RwLock<Node>>];

    fn register_provider_for_link_by_name(&mut self, name: &str, o: Arc<RwLock<Node>>) {
        self.register_provider_for_link(vxunwrap!(self.get_input_link_index_by_name(name)), o);
    }

    fn register_provider_for_link_by_id(&mut self, id: LinkId, o: Arc<RwLock<Node>>) {
        self.register_provider_for_link(vxunwrap!(self.get_input_link_index_by_id(id)), o);
    }

    fn register_provider_for_link(&mut self, usize, Arc<RwLock<Node>>);
}
