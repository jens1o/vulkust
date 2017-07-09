pub mod perspective;

use super::super::math::number::Float;
use super::super::math::matrix::{Mat4x4, Mat3x3};
use super::super::math::vector::Vec3;

pub trait Camera<E>
where
    E: Float,
{
    fn travel(&mut self, _d: &Vec3<E>) {
        logf!("Unimplemented");
    }
    fn place(&mut self, _l: &Vec3<E>) {
        logf!("Unimplemented");
    }
    fn rotate_local_x(&mut self) {
        logf!("Unimplemented");
    }
    fn rotate_local_y(&mut self) {
        logf!("Unimplemented");
    }
    fn rotate_local_z(&mut self) {
        logf!("Unimplemented");
    }
    fn rotate(&mut self, _axis: &Vec3<E>) {
        logf!("Unimplemented");
    }
    fn side(&mut self) {
        logf!("Unimplemented");
    }
    fn forward(&mut self) {
        logf!("Unimplemented");
    }
    fn set_viewport(&mut self, _w: E, _h: E) {
        logf!("Unimplemented");
    }
    fn set_fild_of_view(&mut self, _f: E) {
        logf!("Unimplemented");
    }
    fn set_range(&mut self, _s: E, _e: E) {
        logf!("Unimplemented");
    }
    fn get_view(&self) -> &Mat4x4<E> {
        logf!("Unimplemented");
    }
    fn get_view_projection(&self) -> &Mat4x4<E> {
        logf!("Unimplemented");
    }
    fn look_at(&mut self, eye: &Vec3<E>, at: &Vec3<E>, up: &Vec3<E>) {
        logf!("Unimplemented");
    }
    fn set_rotation_speed(&mut self, speed: E) {
        logf!("Unimplemented");
    }
}

struct Basic<E>
where
    E: Float,
{
    pub speed: E,
    pub rotation_speed: E,
    pos: Vec3<E>,
    lx: Vec3<E>,
    ly: Vec3<E>,
    lz: Vec3<E>,
    r: Mat3x3<E>,
    v: Mat4x4<E>,
}


impl<E> Basic<E>
where
    E: Float,
{
    pub fn new() -> Self {
        Basic {
            speed: E::new(0.01),
            rotation_speed: E::new(0.1),
            pos: Vec3 {
                x: E::new(0.0),
                y: E::new(0.0),
                z: E::new(0.0),
            },
            lx: Vec3 {
                x: E::new(1.0),
                y: E::new(0.0),
                z: E::new(0.0),
            },
            ly: Vec3 {
                x: E::new(0.0),
                y: E::new(1.0),
                z: E::new(0.0),
            },
            lz: Vec3 {
                x: E::new(0.0),
                y: E::new(0.0),
                z: E::new(1.0),
            },
            r: Mat3x3::ident(),
            v: Mat4x4::ident(),
        }
    }
}

impl<E> Camera<E> for Basic<E>
where
    E: Float,
{
    fn travel(&mut self, d: &Vec3<E>) {
        self.pos += d;
        self.v.translate(&-d);
    }

    fn place(&mut self, l: &Vec3<E>) {
        self.pos = *l;
        self.v.set_translation(&l);
    }

    fn rotate_local_x(&mut self) {
        let r = Mat4x4::rotation(-self.rotation_speed, &self.lx);
        let rr = Mat3x3::rotation(self.rotation_speed, &self.lx);
        self.ly = &rr * &self.ly;
        self.lz = &rr * &self.lz;
        self.v = &r * &self.v;
    }

    fn rotate_local_y(&mut self) {
        let r = Mat4x4::rotation(-self.rotation_speed, &self.ly);
        let rr = Mat3x3::rotation(self.rotation_speed, &self.ly);
        self.lx = &rr * &self.lx;
        self.lz = &rr * &self.lz;
        self.v = &r * &self.v;
    }

    fn rotate_local_z(&mut self) {
        let r = Mat4x4::rotation(-self.rotation_speed, &self.lz);
        let rr = Mat3x3::rotation(self.rotation_speed, &self.lz);
        self.lx = &rr * &self.lx;
        self.ly = &rr * &self.ly;
        self.v = &r * &self.v;
    }

    fn rotate(&mut self, axis: &Vec3<E>) {
        let r = Mat4x4::rotation(-self.rotation_speed, axis);
        let rr = Mat3x3::rotation(self.rotation_speed, axis);
        self.lx = &rr * &self.lx;
        self.ly = &rr * &self.ly;
        self.lz = &rr * &self.lz;
        self.v = &r * &self.v;
    }

    fn side(&mut self) {
        let d = &self.lx * self.speed;
        self.travel(&d);
    }

    fn forward(&mut self) {
        let d = &self.lz * self.speed;
        self.travel(&d);
    }

    fn get_view(&self) -> &Mat4x4<E> {
        &self.v
    }

    fn look_at(&mut self, eye: &Vec3<E>, at: &Vec3<E>, up: &Vec3<E>) {
        self.lz = (at - eye).normalized();
        self.lx = self.lz.cross(up).normalized();
        self.ly = self.lx.cross(&self.lz);
        self.pos = *eye;
        self.v = Mat4x4 {
            data: [
                [self.lx.x,          self.ly.x,       -self.lz.x,        E::new(0.0)],
                [self.lx.y,          self.ly.y,       -self.lz.y,        E::new(0.0)],
                [self.lx.z,          self.ly.z,       -self.lz.z,        E::new(0.0)],
                [-self.lx.dot(eye), -self.ly.dot(eye), self.lz.dot(eye), E::new(1.0)],
            ],
        };
        self.r = self.v.get_mat3x3();
    }

    fn set_rotation_speed(&mut self, speed: E) {
        self.rotation_speed = speed;
    }
}
