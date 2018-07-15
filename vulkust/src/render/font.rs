use super::super::core::object::{Base as ObjectBase, Object as CoreObject};
use super::super::core::types::Id;
use super::super::system::file::File;
use std::sync::{Arc, RwLock, Weak};
use std::collections::BTreeMap;
use std::default::Default;
use std::io::Read;
use rusttype::Font as TypeFont;

pub trait Font: CoreObject {
    fn get_font(&self) -> &TypeFont;
}

pub struct Manager {
    pub fonts: BTreeMap<Id, Weak<RwLock<Font>>>,
    pub name_to_id: BTreeMap<String, Id>,
    pub default: Arc<RwLock<Base>>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            fonts: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            default: Arc::new(RwLock::new(Base::default())),
        }
    }

    pub fn load_ttf(&mut self, name: &str) -> Arc<RwLock<Font>> {
        if let Some(font) = self.name_to_id.get(name) {
            if let Some(font) = self.fonts.get(font) {
                if let Some(font) = font.upgrade() {
                    return font;
                }
            }
        }
        let font = Base::new_ttf(name);
        let name = name.to_string();
        let id = font.get_id();
        self.name_to_id.insert(name, id);
        let font: Arc<RwLock<Font>> = Arc::new(RwLock::new(font));
        self.fonts.insert(id, Arc::downgrade(&font));
        return font;
    }
}

pub struct Base {
    pub obj_base: ObjectBase,
    pub name: Option<String>,
    pub font: TypeFont<'static>,
}

impl Base {
    pub fn new_ttf(name: &str) -> Self {
        if !name.ends_with(".ttf") {
            vxlogf!("Only TTF file is acceptable, not {}.", name);
        }
        let file = format!("data/fonts/{}", name);
        let mut file = vxresult!(File::open(&file));
        let mut data = Vec::new();
        vxresult!(file.read_to_end(&mut data));
        let font = vxresult!(TypeFont::from_bytes(data));
        let obj_base = ObjectBase::new();
        let name = Some(name.to_string());
        Base {
            obj_base,
            name,
            font,
        }
    }
}

impl Default for Base {
    fn default() -> Self {
        let font = include_bytes!(concat!(env!("OUT_DIR"), 
            "/render/fonts/Ubuntu-B.ttf"));
        let font = vxresult!(TypeFont::from_bytes(font));
        let obj_base = ObjectBase::new();
        let name = None;
        Base {
            obj_base,
            name,
            font,
        }
    }
}

impl CoreObject for Base {
    fn get_id(&self) -> Id {
        self.obj_base.get_id()
    }
}

impl Font for Base {
    fn get_font(&self) -> &TypeFont {
        &self.font
    }
}