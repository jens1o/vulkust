use super::super::core::object::Object as CoreObject;
use super::super::core::types::Id;
use super::super::physics::collider::{read as read_collider, Collider, Ghost as GhostCollider};
use super::buffer::DynamicBuffer;
use super::engine::Engine;
use super::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::mesh::{Base as MeshBase, Mesh};
use super::object::{Base as ObjectBase, Loadable, Object};
use super::texture::{Loadable as TextureLoadable, Texture2D};
use std::collections::BTreeMap;
use std::mem::size_of;
use std::sync::{Arc, RwLock, Weak};

use gltf;
use math;

pub trait Model: Object {}

#[repr(u8)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum TypeId {
    Dynamic = 1,
    Static = 2,
    Widget = 3,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Manager {
    pub models: BTreeMap<Id, Weak<RwLock<Model>>>,
    pub name_to_id: BTreeMap<String, Id>,
    pub gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            models: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            gx3d_table: None,
        }
    }

    pub fn load_gx3d(&mut self, engine: &Arc<RwLock<Engine>>, id: Id) -> Arc<RwLock<Model>> {
        if let Some(model) = self.models.get(&id) {
            if let Some(model) = model.upgrade() {
                return model;
            }
        }
        let gx3d_table = vxunwrap!(self.gx3d_table.as_mut());
        gx3d_table.goto(id);
        let reader = &mut gx3d_table.reader;
        let t = reader.read_type_id();
        let model: Arc<RwLock<Model>> = if t == TypeId::Static as u8 {
            Arc::new(RwLock::new(Base::new_with_gx3d(engine, reader, id)))
        } else if t == TypeId::Dynamic as u8 {
            vxunimplemented!()
        } else if t == TypeId::Widget as u8 {
            vxunimplemented!()
        } else {
            vxunexpected!()
        };
        self.models.insert(id, Arc::downgrade(&model));
        return model;
    }
}

#[repr(C)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Uniform {
    pub model: math::Matrix4<f32>,
    pub model_view_projection: math::Matrix4<f32>,
    // todo, I think its not gonna be needed,
    // because of cascaded shadow
    // pub directional_biased_model: math::Matrix4<f32>,
    // pub sun_mvp: math::Matrix4<f32>,
}

impl Uniform {
    fn new_with_gltf(node: &gltf::Node) -> Self {
        let m = node.transform().matrix();
        let model = math::Matrix4::new(
            m[0][0], m[0][1], m[0][2], m[0][3], m[1][0], m[1][1], m[1][2], m[1][3], m[2][0],
            m[2][1], m[2][2], m[2][3], m[3][0], m[3][1], m[3][2], m[3][3],
        );
        Uniform {
            model,
            model_view_projection: model,
        }
    }

    fn new_with_gx3d(reader: &mut Gx3DReader) -> Self {
        let model = math::Matrix4::new(
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
        );
        Uniform {
            model,
            model_view_projection: model,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Base {
    pub obj_base: ObjectBase,
    pub is_dynamic: bool,
    pub has_shadow_caster: bool,
    pub has_transparent: bool,
    pub occlusion_culling_radius: f32,
    pub is_in_sun: Vec<bool>,
    pub is_in_camera: Vec<bool>,
    pub distance_from_cameras: Vec<f32>,
    pub collider: Arc<RwLock<Collider>>,
    pub uniform: Uniform,
    pub uniform_buffer: DynamicBuffer,
    pub meshes: BTreeMap<Id, Arc<RwLock<Mesh>>>,
    pub children: BTreeMap<Id, Arc<RwLock<Model>>>,
}

impl Base {}

impl CoreObject for Base {
    fn get_id(&self) -> Id {
        self.obj_base.get_id()
    }
}

impl Object for Base {
    fn get_name(&self) -> Option<String> {
        self.obj_base.get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.obj_base.get_name();
        vxunimplemented!();
    }

    fn render(&self, engine: &Engine) {
        if !self.obj_base.renderable {
            return;
        }
        self.obj_base.render(engine);
        vxunimplemented!();
    }

    fn disable_rendering(&mut self) {
        self.obj_base.disable_rendering();
    }

    fn enable_rendering(&mut self) {
        self.obj_base.enable_rendering();
    }

    fn update(&mut self) {
        vxunimplemented!();
    }
}

impl Loadable for Base {
    fn new_with_gltf(node: &gltf::Node, eng: &Arc<RwLock<Engine>>, data: &[u8]) -> Self {
        let obj_base = ObjectBase::new();
        let engine = vxresult!(eng.read());
        let scene_manager = vxresult!(engine.scene_manager.read());
        let mut mesh_manager = vxresult!(scene_manager.mesh_manager.write());
        let model = vxunwrap!(node.mesh());
        let primitives = model.primitives();
        let mut meshes = BTreeMap::new();
        let mut has_shadow_caster = false;
        let mut has_transparent = false;
        let mut occlusion_culling_radius = 0.0001;
        for primitive in primitives {
            let mesh = mesh_manager.load_gltf(primitive, &engine, data);
            let id = {
                let mesh = vxresult!(mesh.read());
                has_shadow_caster |= mesh.is_shadow_caster();
                has_transparent |= mesh.is_transparent();
                let occ = mesh.get_occlusion_culling_radius();
                if occ > occlusion_culling_radius {
                    occlusion_culling_radius = occ;
                }
                mesh.get_id()
            };
            meshes.insert(id, mesh);
        }
        if node.children().count() > 0 {
            vxunimplemented!(); // todo support children
        }
        let gapi_engine = vxresult!(engine.gapi_engine.read());
        let uniform_buffer = vxresult!(gapi_engine.buffer_manager.write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        Base {
            obj_base,
            is_dynamic: true,
            has_shadow_caster,
            has_transparent,
            occlusion_culling_radius,
            is_in_sun: Vec::new(),
            is_in_camera: Vec::new(),
            distance_from_cameras: Vec::new(),
            collider: Arc::new(RwLock::new(GhostCollider::new())),
            uniform: Uniform::new_with_gltf(node),
            uniform_buffer,
            meshes,
            children: BTreeMap::new(),
        }
    }

    fn new_with_gx3d(engine: &Arc<RwLock<Engine>>, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let obj_base = ObjectBase::new_with_id(my_id);
        let uniform = Uniform::new_with_gx3d(reader);
        let occlusion_culling_radius = reader.read();
        let collider = read_collider(reader);
        let meshes_ids = reader.read_array();
        let eng = vxresult!(engine.read());
        let scene_manager = vxresult!(eng.scene_manager.read());
        let mut mesh_manager = vxresult!(scene_manager.mesh_manager.write());
        let mut meshes = BTreeMap::new();
        let mut has_shadow_caster = false;
        let mut has_transparent = false;
        for mesh_id in meshes_ids {
            let mesh = mesh_manager.load_gx3d(engine, mesh_id);
            {
                let mesh = vxresult!(mesh.read());
                has_shadow_caster |= mesh.is_shadow_caster();
                has_transparent |= mesh.is_transparent();
            }
            meshes.insert(mesh_id, mesh);
        }
        let gapi_engine = vxresult!(eng.gapi_engine.read());
        let uniform_buffer = vxresult!(gapi_engine.buffer_manager.write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        Base {
            obj_base,
            is_dynamic: false, // todo there must be a dynamic struct for this that implement transformable
            has_shadow_caster,
            has_transparent,
            occlusion_culling_radius,
            is_in_sun: Vec::new(),
            is_in_camera: Vec::new(),
            distance_from_cameras: Vec::new(),
            collider,
            uniform,
            uniform_buffer,
            meshes,
            children: BTreeMap::new(),
        }
    }
}

impl Model for Base {}
