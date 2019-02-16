pub mod g_buffer_filler;
pub mod shadow_accumulator;
pub mod shadow_mapper;
pub mod ssao;
pub mod ssr;

use super::super::super::core::debug::Debug as CoreDebug;
use super::super::super::core::types::Id;
use super::super::engine::Engine;
use super::super::gapi::GraphicApiEngine;
use super::super::scene::Scene;
use super::super::texture::Texture;
use std::collections::BTreeMap;
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
/// It must create a new

pub type LinkId = Id;

const POSITION: LinkId = 1;
const POSITION_NAME: &'static str = "position";

const NORMAL: LinkId = 2;
const NORMAL_NAME: &'static str = "normal";

const TANGENT: LinkId = 3;
const TANGENT_NAME: &'static str = "tangent";

const BITANGENT: LinkId = 4;
const BITANGENT_NAME: &'static str = "bitangent";

const DEPTH: LinkId = 5;
const DEPTH_NAME: &'static str = "depth";

const OCCLUSION: LinkId = 6;
const OCCLUSION_NAME: &'static str = "occlusion";

const SINGLE_INPUT: LinkId = 7;
const SINGLE_INPUT_NAME: &'static str = "single-input";

const SINGLE_OUTPUT: LinkId = 8;
const SINGLE_OUTPUT_NAME: &'static str = "single-output";

const ALBEDO: LinkId = 9;
const ALBEDO_NAME: &'static str = "albedo";

pub trait Node: CoreDebug {
    /// Implementor of trait either can provide this methode or it must implement all other default implementations
    fn get_base(&self) -> &Base {
        vxunexpected!();
    }

    fn get_mut_base(&mut self) -> &mut Base {
        vxunexpected!();
    }

    fn create_new(&self, geng: &GraphicApiEngine) -> Arc<RwLock<Node>>;
    fn get_output_texture(&self, usize) -> &Arc<RwLock<Texture>>;

    fn register_consumer_for_link(&mut self, index: usize, c: Weak<RwLock<Node>>) {
        self.get_mut_base().register_consumer_for_link(index, c);
    }

    fn register_provider_for_link(&mut self, index: usize, p: Arc<RwLock<Node>>) {
        self.get_mut_base().register_provider_for_link(index, p);
    }

    fn register_consumer_for_link_by_name(&mut self, name: &str, o: Weak<RwLock<Node>>) {
        self.register_consumer_for_link(
            vxunwrap!(self.get_base().get_output_link_index_by_name(name)),
            o,
        );
    }

    fn register_consumer_for_link_by_id(&mut self, id: LinkId, o: Weak<RwLock<Node>>) {
        self.register_consumer_for_link(
            vxunwrap!(self.get_base().get_output_link_index_by_id(id)),
            o,
        );
    }

    fn register_provider_for_link_by_name(&mut self, name: &str, o: Arc<RwLock<Node>>) {
        self.register_provider_for_link(
            vxunwrap!(self.get_base().get_input_link_index_by_name(name)),
            o,
        );
    }

    fn register_provider_for_link_by_id(&mut self, id: LinkId, o: Arc<RwLock<Node>>) {
        self.register_provider_for_link(
            vxunwrap!(self.get_base().get_input_link_index_by_id(id)),
            o,
        );
    }

    fn get_output_texture_by_name(&mut self, name: &str) -> &Arc<RwLock<Texture>> {
        self.get_output_texture(vxunwrap!(self
            .get_base()
            .get_output_link_index_by_name(name)))
    }

    fn get_output_texture_by_id(&mut self, id: LinkId) -> &Arc<RwLock<Texture>> {
        self.get_output_texture(vxunwrap!(self.get_base().get_output_link_index_by_id(id)))
    }

    fn get_name(&self) -> &str {
        self.get_base().get_name()
    }

    fn get_input_links_names(&self) -> &[String] {
        self.get_base().get_input_links_names()
    }

    fn get_input_links_ids(&self) -> &[LinkId] {
        self.get_base().get_input_links_ids()
    }

    fn get_input_link_index_by_name(&self, name: &str) -> Option<usize> {
        self.get_base().get_input_link_index_by_name(name)
    }

    fn get_input_link_index_by_id(&self, id: LinkId) -> Option<usize> {
        self.get_base().get_input_link_index_by_id(id)
    }

    fn get_output_links_names(&self) -> &[String] {
        self.get_base().get_output_links_names()
    }

    fn get_output_links_ids(&self) -> &[LinkId] {
        self.get_base().get_output_links_ids()
    }

    fn get_output_link_index_by_name(&self, name: &str) -> Option<usize> {
        self.get_base().get_output_link_index_by_name(name)
    }

    fn get_output_link_index_by_id(&self, id: LinkId) -> Option<usize> {
        self.get_base().get_output_link_index_by_id(id)
    }

    fn get_link_consumers(&self, index: usize) -> &[Weak<RwLock<Node>>] {
        self.get_base().get_link_consumers(index)
    }

    fn get_all_consumers(&self) -> &[Vec<Weak<RwLock<Node>>>] {
        self.get_base().get_all_consumers()
    }

    fn get_link_provider(&self, index: usize) -> &Arc<RwLock<Node>> {
        self.get_base().get_link_provider(index)
    }

    fn get_all_providers(&self) -> &[Arc<RwLock<Node>>] {
        self.get_base().get_all_providers()
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Base {
    pub input_links_id_index: BTreeMap<LinkId, usize>,
    pub input_links_name_index: BTreeMap<String, usize>,
    pub output_links_id_index: BTreeMap<LinkId, usize>,
    pub output_links_name_index: BTreeMap<String, usize>,
    pub name: String,
    pub input_links_ids: Vec<LinkId>,
    pub input_links_names: Vec<String>,
    pub output_links_ids: Vec<LinkId>,
    pub output_links_names: Vec<String>,
    pub providers: Vec<Arc<RwLock<Node>>>,
    pub consumers: Vec<Vec<Weak<RwLock<Node>>>>,
}

impl Base {
    pub fn new(
        name: String,
        input_links_names: Vec<String>,
        input_links_ids: Vec<LinkId>,
        output_links_names: Vec<String>,
        output_links_ids: Vec<LinkId>,
    ) -> Self {
        let mut input_links_id_index = BTreeMap::new();
        let mut input_links_name_index = BTreeMap::new();
        let mut output_links_id_index = BTreeMap::new();
        let mut output_links_name_index = BTreeMap::new();
        for i in 0..input_links_ids.len() {
            input_links_id_index.insert(input_links_ids[i], i);
            input_links_name_index.insert(input_links_names[i], i);
        }
        for i in 0..output_links_ids.len() {
            output_links_id_index.insert(output_links_ids[i], i);
            output_links_name_index.insert(output_links_names[i], i);
        }
        let providers = Vec::new();
        let consumers = Vec::new();
        Self {
            name,
            input_links_names,
            input_links_ids,
            output_links_names,
            output_links_ids,
            input_links_id_index,
            input_links_name_index,
            output_links_id_index,
            output_links_name_index,
            providers,
            consumers,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_input_links_names(&self) -> &[String] {
        &self.input_links_names
    }

    pub fn get_input_links_ids(&self) -> &[LinkId] {
        &self.input_links_ids
    }

    pub fn get_input_link_index_by_name(&self, name: &str) -> Option<usize> {
        let mut i = 0;
        for l in &self.input_links_names {
            if name == l {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    pub fn get_input_link_index_by_id(&self, id: LinkId) -> Option<usize> {
        let mut i = 0;
        for l in &self.input_links_ids {
            if id == *l {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    pub fn get_output_links_names(&self) -> &[String] {
        &self.output_links_names
    }

    pub fn get_output_links_ids(&self) -> &[LinkId] {
        &self.output_links_ids
    }

    pub fn get_output_link_index_by_name(&self, name: &str) -> Option<usize> {
        let mut i = 0;
        for l in &self.output_links_names {
            if name == l {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    pub fn get_output_link_index_by_id(&self, id: LinkId) -> Option<usize> {
        let mut i = 0;
        for l in &self.output_links_ids {
            if id == *l {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    pub fn get_link_consumers(&self, index: usize) -> &[Weak<RwLock<Node>>] {
        &self.consumers[index]
    }

    pub fn get_all_consumers(&self) -> &[Vec<Weak<RwLock<Node>>>] {
        &self.consumers
    }

    pub fn get_link_provider(&self, index: usize) -> &Arc<RwLock<Node>> {
        &self.providers[index]
    }

    pub fn get_all_providers(&self) -> &[Arc<RwLock<Node>>] {
        &self.providers
    }

    pub fn create_new(&self) -> Self {
        Self {
            input_links_id_index: self.input_links_id_index.clone(),
            input_links_name_index: self.input_links_name_index.clone(),
            output_links_id_index: self.output_links_id_index.clone(),
            output_links_name_index: self.output_links_name_index.clone(),
            name: self.name.clone(),
            input_links_ids: self.input_links_ids.clone(),
            input_links_names: self.input_links_names.clone(),
            output_links_ids: self.output_links_ids.clone(),
            output_links_names: self.output_links_names.clone(),
            providers: Vec::new(),
            consumers: Vec::new(),
        }
    }

    pub fn register_consumer_for_link(&mut self, index: usize, c: Weak<RwLock<Node>>) {
        self.consumers[index].push(c);
    }

    pub fn register_provider_for_link(&mut self, index: usize, p: Arc<RwLock<Node>>) {
        self.providers[index] = p;
    }
}
