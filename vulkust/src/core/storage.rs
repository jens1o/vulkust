use super::object::Object;
use super::types::Id;
use std::collections::BTreeMap;
use std::sync::{Arc, Weak};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Storage<T>
where
    T: Object + ?Sized,
{
    id: BTreeMap<Id, Arc<T>>,
    name: BTreeMap<String, Arc<T>>,
}

impl<T> Storage<T>
where
    T: Object + ?Sized,
{
    pub fn new() -> Self {
        Self {
            id: BTreeMap::new(),
            name: BTreeMap::new(),
        }
    }

    #[inline]
    pub fn insert(&mut self, t: Arc<T>) {
        let id = t.get_id();
        #[cfg(debug_mode)]
        {
            if self.id.contains_key(&id) {
                vxlogf!("Other object with id: {:?} is already inserted.", id);
            }
        }
        self.id.insert(id, t.clone());
        if let Some(name) = t.get_name() {
            #[cfg(debug_mode)]
            {
                if self.name.contains_key(name) {
                    vxlogf!("Other object with name: {:?} is already inserted.", id);
                }
            }
            let name = name.to_string();
            self.name.insert(name, t);
        }
    }

    #[inline]
    pub fn get_with_id(&self, id: Id) -> Option<&Arc<T>> {
        self.id.get(&id)
    }

    #[inline]
    pub fn get_with_name(&mut self, name: &str) -> Option<&Arc<T>> {
        self.name.get(name)
    }

    #[inline]
    pub fn delete_with_id(&self, id: &Id) {
        if let Some(t) = self.id.remove(id) {
            if let Some(name) = t.get_name() {
                self.name.remove(name);
            }
        }
    }

    #[inline]
    pub fn delete_with_name(&self, name: &str) {
        if let Some(t) = self.name.remove(name) {
            self.id.remove(&t.get_id());
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct WeakStorage<T>
where
    T: ?Sized + Object,
{
    id: BTreeMap<Id, Weak<T>>,
    name: BTreeMap<String, Weak<T>>,
}

impl<T> WeakStorage<T>
where
    T: ?Sized + Object,
{
    pub fn new() -> Self {
        Self {
            id: BTreeMap::new(),
            name: BTreeMap::new(),
        }
    }

    #[inline]
    pub fn insert(&mut self, t: &Arc<T>) {
        let id = t.get_id();
        #[cfg(debug_mode)]
        {
            if self.id.contains_key(&id) {
                vxlogf!("Other object with id: {:?} is already inserted.", id);
            }
        }
        self.id.insert(id, Arc::downgrade(t));
        if let Some(name) = t.get_name() {
            #[cfg(debug_mode)]
            {
                if self.name.contains_key(name) {
                    vxlogf!("Other object with name: {:?} is already inserted.", id);
                }
            }
            let name = name.to_string();
            self.name.insert(name, Arc::downgrade(t));
        }
    }

    #[inline]
    pub fn get_with_id(&self, id: Id) -> Option<Arc<T>> {
        if let Some(t) = self.id.get(&id) {
            if let Some(t) = t.upgrade() {
                return Some(t);
            }
        }
        None
    }

    #[inline]
    pub fn get_with_name(&self, name: &str) -> Option<Arc<T>> {
        if let Some(t) = self.name.get(name) {
            if let Some(t) = t.upgrade() {
                return Some(t);
            }
        }
        None
    }

    pub fn clean(&mut self) {
        let mut ids = Vec::with_capacity(self.id.len());
        for (id, t) in &self.id {
            if t.upgrade().is_none() {
                ids.push(id);
            }
        }
        for id in ids {
            self.id.remove(id);
        }
        let mut names = Vec::with_capacity(self.name.len());
        for (name, t) in &self.name {
            if t.upgrade().is_none() {
                names.push(name);
            }
        }
        for name in names {
            self.name.remove(name);
        }
    }
}
