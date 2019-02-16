use super::super::super::super::core::types::Id;
use super::super::super::super::core::storage::WeakStorage;
use super::super::super::gapi::GraphicApiEngine;
use super::Node;
use std::sync::{RwLock, Arc};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Manager {
    storage: WeakStorage<RwLock<Node>>,
}

impl Manager {
    pub(crate) fn new() -> Self {
        Self {
            storage: WeakStorage::new();
        }
    }

    pub fn get_with_index(&self, i: usize, geng: &GraphicApiEngine) -> Option<Arc<RwLock<Node>>> {
        if let Some(n) = self.storage.get_with_index(i) {
            Some(vxresult!(n.read()).create_new(geng))
        } else {
            None
        }
    }

    pub fn get_with_id(&self, id: Id, geng: &GraphicApiEngine) -> Option<Arc<RwLock<Node>>> {
        if let Some(n) = self.storage.get_with_id(id) {
            Some(vxresult!(n.read()).create_new(geng))
        } else {
            None
        }
    }

    pub fn get_with_name(&self, name: &str, geng: &GraphicApiEngine) -> Option<Arc<RwLock<Node>>> {
        if let Some(n) = self.storage.get_with_name(name) {
            Some(vxresult!(n.read()).create_new(geng))
        } else {
            None
        }
    }
}