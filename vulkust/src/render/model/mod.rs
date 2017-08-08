pub mod manager;

use std::sync::Arc;
use std::cell::RefCell;
use super::super::core::application::ApplicationTrait;
use super::super::math::matrix::Mat4x4;
use super::super::system::os::OsApplication;
use super::super::system::file::File;
use super::buffer::Buffer;
use super::mesh::Mesh;

pub trait Model {}

pub struct StaticModel {
    pub draw_mesh: Mesh,
    pub children: Vec<Box<Model>>,
}

impl StaticModel {
    pub fn new<CoreApp>(
        file: &mut File,
        os_app: &mut OsApplication<CoreApp>,
        vertices_buffer: &mut Buffer,
        indices_buffer: &mut Buffer) -> Self
    where
        CoreApp: ApplicationTrait,
    {
        let mesh = Mesh::new(file, os_app, vertices_buffer, indices_buffer);
        let children_count: u64 = file.read_type();
        let mut children = Vec::new();
        for _ in 0..children_count {
            children.push(read_boxed_model(file, os_app, vertices_buffer, indices_buffer));
        }
        StaticModel {
            draw_mesh: mesh,
            children: children,
        }
    }
}

impl Model for StaticModel {}

pub struct DynamicModel {
    pub transform: Mat4x4<f32>,
    pub occ_mesh: Mesh,
    pub children: Vec<Box<Model>>,
}

impl DynamicModel {
    pub fn new<CoreApp>(
        file: &mut File, os_app: &mut OsApplication<CoreApp>,
        vertices_buffer: &mut Buffer,
        indices_buffer: &mut Buffer) -> Self
    where
        CoreApp: ApplicationTrait,
    {
        let m = Mat4x4::new_from_file(file);
        let mesh = Mesh::new(file, os_app, vertices_buffer, indices_buffer);
        let children_count: u64 = file.read_type();
        let mut children = Vec::new();
        for _ in 0..children_count {
            children.push(read_boxed_model(file, os_app, vertices_buffer, indices_buffer));
        }
        DynamicModel {
            transform: m,
            occ_mesh: mesh,
            children: children,
        }
    }
}

impl Model for DynamicModel {}

pub struct CopyModel {
    pub t: Mat4x4<f32>,
    pub sm: Arc<RefCell<Model>>,
}

impl CopyModel {
    pub fn new<CoreApp>(
        file: &mut File, os_app: &mut OsApplication<CoreApp>,
        vertices_buffer: &mut Buffer,
        indices_buffer: &mut Buffer) -> Self
    where
        CoreApp: ApplicationTrait,
    {
        let t = Mat4x4::new_from_file(file);
        let id = file.read_id();
        CopyModel {
            t: t,
            sm: os_app.asset_manager.get_model(id, os_app, vertices_buffer, indices_buffer),
        }
    }
}

impl Model for CopyModel {}

pub fn read_model<CoreApp>(
    file: &mut File,
    os_app: &mut OsApplication<CoreApp>,
    vertices_buffer: &mut Buffer,
    indices_buffer: &mut Buffer,
) -> Arc<RefCell<Model>>
where
    CoreApp: ApplicationTrait,
{
    return if file.read_bool() {
        Arc::new(RefCell::new(CopyModel::new(file, os_app, vertices_buffer, indices_buffer)))
    } else if file.read_bool() {
        Arc::new(RefCell::new(DynamicModel::new(file, os_app, vertices_buffer, indices_buffer)))
    } else {
        Arc::new(RefCell::new(StaticModel::new(file, os_app, vertices_buffer, indices_buffer)))
    };
}

fn read_boxed_model<CoreApp>(
    file: &mut File,
    os_app: &mut OsApplication<CoreApp>,
    vertices_buffer: &mut Buffer,
    indices_buffer: &mut Buffer,
) -> Box<Model>
where
    CoreApp: ApplicationTrait,
{
    return if file.read_bool() {
        Box::new(CopyModel::new(file, os_app, vertices_buffer, indices_buffer))
    } else if file.read_bool() {
        Box::new(DynamicModel::new(file, os_app, vertices_buffer, indices_buffer))
    } else {
        Box::new(StaticModel::new(file, os_app, vertices_buffer, indices_buffer))
    };
}
