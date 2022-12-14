use std::{
    f32::consts::PI,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn length_squared(&self) -> f32 {
        return self.x * self.x + self.y * self.y;
    }
    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        return self.x * rhs.x + self.y * rhs.y;
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn pow(&self, n: f32) -> Self {
        return Vec3 {
            x: f32::powf(self.x, n),
            y: f32::powf(self.y, n),
            z: f32::powf(self.z, n),
        };
    }

    pub fn scale(&self, s: f32) -> Self {
        return Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        };
    }

    pub fn length_squared(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        Vec3::new(
            self.x * 1.0 / self.length(),
            self.y * 1.0 / self.length(),
            self.z * 1.0 / self.length(),
        )
    }

    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 { x, y, z, w }
    }

    pub fn from_vec3(vec3: &Vec3, w: f32) -> Self {
        Vec4 {
            x: vec3.x,
            y: vec3.y,
            z: vec3.z,
            w,
        }
    }

    pub fn dot(&self, rhs: &Vec4) -> f32 {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w;
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vec4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

/**
 * column based mat4 implementation
 */
#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub m: [f32; 16],
}

impl Mat4 {
    pub fn frustum(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Self {
        return Mat4::new([
            (2.0 * n) / (r - l),
            0.0,
            (r + l) / (r - l),
            0.0,
            0.0,
            (2.0 * n) / (t - b),
            (t + b) / (t - b),
            0.0,
            0.0,
            0.0,
            (-(f + n)) / (f - n),
            -1.0,
            0.0,
            0.0,
            (-2.0 * f * n) / (f - n),
            0.0,
        ]);
    }
    pub fn perspective(fov: f32, aspect: f32, znear: f32, zfar: f32) -> Self {
        let ymax = znear * (fov * PI / 360.0f32).tan();
        let xmax = ymax * aspect;
        return Mat4::frustum(-xmax, xmax, -ymax, ymax, znear, zfar);
    }

    pub fn lookat(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let f = (target - eye).normalize().scale(-1.0);
        let r = up.cross(&f).normalize();
        let u = f.cross(&r).normalize();
        let t = Vec3::new(-r.dot(&eye), -u.dot(&eye), -f.dot(&eye));
        Mat4::new([
            r.x, u.x, f.x, 0.0, r.y, u.y, f.y, 0.0, r.z, u.z, f.z, 0.0, t.x, t.y, t.z, 1.0,
        ])
    }
    pub fn identity() -> Self {
        let mut m = [0.0f32; 16];
        m[0] = 1.0;
        m[5] = 1.0;
        m[10] = 1.0;
        m[15] = 1.0;
        Mat4 { m }
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        let mut m = Mat4::identity();
        m.m[12] = x;
        m.m[13] = y;
        m.m[14] = z;
        m
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        let mut m = [0.0f32; 16];
        m[0] = x;
        m[5] = y;
        m[10] = z;
        m[15] = 1.0;
        Mat4 { m }
    }

    pub fn column(&self, col: usize) -> Vec4 {
        return Vec4::new(
            self.m[col * 4],
            self.m[col * 4 + 1],
            self.m[col * 4 + 2],
            self.m[col * 4 + 3],
        );
    }

    pub fn row(&self, row: usize) -> Vec4 {
        return Vec4::new(
            self.m[row],
            self.m[row + 4],
            self.m[row + 8],
            self.m[row + 12],
        );
    }

    pub fn minor3x3(
        &self,
        c0: usize,
        c1: usize,
        c2: usize,
        r0: usize,
        r1: usize,
        r2: usize,
    ) -> f32 {
        self.m[c0 * 4 + r0]
            * (self.m[c1 * 4 + r1] * self.m[c2 * 4 + r2]
                - self.m[c1 * 4 + r2] * self.m[c2 * 4 + r1])
            - self.m[c1 * 4 + r0]
                * (self.m[c0 * 4 + r1] * self.m[c2 * 4 + r2]
                    - self.m[c0 * 4 + r2] * self.m[c2 * 4 + r1])
            + self.m[c2 * 4 + r0]
                * (self.m[c0 * 4 + r1] * self.m[c1 * 4 + r2]
                    - self.m[c0 * 4 + r2] * self.m[c1 * 4 + r1])
    }

    pub fn new(m: [f32; 16]) -> Self {
        Mat4 { m }
    }

    pub fn transpose(&self) -> Self {
        let mut c = Mat4::new(self.m);
        c.m.swap(1, 4);
        c.m.swap(2, 8);
        c.m.swap(3, 12);
        c.m.swap(6, 9);
        c.m.swap(7, 13);
        c.m.swap(11, 14);
        c
    }

    pub fn inverse(&self) -> Self {
        let det = self.determinant();

        if (det == 0.0) {
            return Mat4::identity();
        }

        let adj = self.adjugate();
        return adj * (1.0 / det);
    }

    pub fn determinant(&self) -> f32 {
        self.m[0] * self.minor3x3(1, 2, 3, 1, 2, 3) - self.m[4] * self.minor3x3(0, 2, 3, 1, 2, 3)
            + self.m[8] * self.minor3x3(0, 1, 3, 1, 2, 3)
            - self.m[12] * self.minor3x3(0, 1, 2, 1, 2, 3)
    }

    pub fn adjugate(&self) -> Mat4 {
        let mut res = Mat4::identity();

        res.m[0] = self.minor3x3(1, 2, 3, 1, 2, 3);
        res.m[1] = -self.minor3x3(1, 2, 3, 0, 2, 3);
        res.m[2] = self.minor3x3(1, 2, 3, 0, 1, 3);
        res.m[3] = -self.minor3x3(1, 2, 3, 0, 1, 2);

        res.m[4] = -self.minor3x3(0, 2, 3, 1, 2, 3);
        res.m[5] = self.minor3x3(0, 2, 3, 0, 2, 3);
        res.m[6] = -self.minor3x3(0, 2, 3, 0, 1, 3);
        res.m[7] = self.minor3x3(0, 2, 3, 0, 1, 2);

        res.m[8] = self.minor3x3(0, 1, 3, 1, 2, 3);
        res.m[9] = -self.minor3x3(0, 1, 3, 0, 2, 3);
        res.m[10] = self.minor3x3(0, 1, 3, 0, 1, 3);
        res.m[11] = -self.minor3x3(0, 1, 3, 0, 1, 2);

        res.m[12] = -self.minor3x3(0, 1, 2, 1, 2, 3);
        res.m[13] = self.minor3x3(0, 1, 2, 0, 2, 3);
        res.m[14] = -self.minor3x3(0, 1, 2, 0, 1, 3);
        res.m[15] = self.minor3x3(0, 1, 2, 0, 1, 2);

        res.transpose()
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        return self.column(0) * rhs.x
            + self.column(1) * rhs.y
            + self.column(2) * rhs.z
            + self.column(3) * rhs.w;

        // self.column(0) + self.column(1) * rhs.y + self.column(2) * rhs.z + self.column(3) * rhs.w;
    }
}

impl Mul<f32> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut m = self.m.clone();
        for i in 0..m.len() {
            m[i] *= rhs;
        }
        return Mat4::new(m);
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        let m = [
            self.row(0).dot(&rhs.column(0)),
            self.row(1).dot(&rhs.column(0)),
            self.row(2).dot(&rhs.column(0)),
            self.row(3).dot(&rhs.column(0)),
            self.row(0).dot(&rhs.column(1)),
            self.row(1).dot(&rhs.column(1)),
            self.row(2).dot(&rhs.column(1)),
            self.row(3).dot(&rhs.column(1)),
            self.row(0).dot(&rhs.column(2)),
            self.row(1).dot(&rhs.column(2)),
            self.row(2).dot(&rhs.column(2)),
            self.row(3).dot(&rhs.column(2)),
            self.row(0).dot(&rhs.column(3)),
            self.row(1).dot(&rhs.column(3)),
            self.row(2).dot(&rhs.column(3)),
            self.row(3).dot(&rhs.column(3)),
        ];
        Mat4 { m }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Quat {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quat {
    pub fn identity() -> Self {
        Quat {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn vector(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }

    pub fn normalize(&self) -> Self {
        return Quat {
            x: self.x / self.length(),
            y: self.y / self.length(),
            z: self.z / self.length(),
            w: self.w / self.length(),
        };
    }

    pub fn conjugate(&self) -> Self {
        Quat {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    pub fn inverse(&self) -> Self {
        self.conjugate().normalize()
    }

    pub fn dot(&self, b: &Self) -> f32 {
        return self.x * b.x + self.y * b.y + self.z * b.z + self.w * b.w;
    }

    pub fn to_mat4(&self) -> Mat4 {
        let r = self.mul(Vec3::new(1.0, 0.0, 0.0));
        let u = self.mul(Vec3::new(0.0, 1.0, 0.0));
        let f = self.mul(Vec3::new(0.0, 0.0, 1.0));

        return Mat4::new([
            r.x, r.y, r.z, 0.0, u.x, u.y, u.z, 0.0, f.x, f.y, f.z, 0.0, 0.0, 0.0, 0.0, 1.0,
        ]);
    }

    pub fn from_angle_axis(angle: f32, axis: Vec3) -> Self {
        let norm = axis.normalize();
        let s = (angle * 0.5).sin();
        Quat {
            x: norm.x * s,
            y: norm.y * s,
            z: norm.z * s,
            w: (angle * 0.5).cos(),
        }
    }
}

impl Mul for Quat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Quat {
            x: rhs.x * self.w + rhs.y * self.z - rhs.z * self.y + rhs.w * self.x,
            y: -rhs.x * self.z + rhs.y * self.w + rhs.z * self.x + rhs.w * self.y,
            z: rhs.x * self.y - rhs.y * self.x + rhs.z * self.w + rhs.w * self.z,
            w: -rhs.x * self.x - rhs.y * self.y - rhs.z * self.z + rhs.w * self.w,
        }
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        return self.vector() * 2.0 * self.vector().dot(&v)
            + v * (self.w * self.w - self.vector().dot(&self.vector()))
            + self.vector().cross(&v) * 2.0 * self.w;
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::Mat4;
    use super::Quat;
    use super::Vec3;
    use super::Vec4;

    use nalgebra::Unit;

    #[test]
    fn test_perspective_mat4() {
        let fov = 60.0 * PI / 180.0;
        let aspect = 4.0 / 3.0;
        let znear = 1.0;
        let zfar = 100.0;
        let perspective1 = Mat4::perspective(60.0, 4.0 / 3.0, 1.0, 100.0);
        let perspective2 = nalgebra::base::Matrix4::new_perspective(aspect, fov, znear, zfar);
        println!("{:?}", perspective1);
        println!("{:?}", perspective2);
    }
    #[test]
    fn test_lookat_mat4() {
        let lookat1 = Mat4::lookat(
            Vec3::new(-1.0, -1.0, 5.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        let lookat2 = nalgebra::base::Matrix4::look_at_rh(
            &nalgebra::Point3::new(-1.0, -1.0, 5.0),
            &nalgebra::Point3::new(1.0, 1.0, 0.0),
            &nalgebra::Vector3::new(0.0, 1.0, 0.0),
        );
        println!("{:?}", lookat1);
        println!("{:?}", lookat2);
    }

    #[test]
    fn test_matrix_mul() {
        let fov = 60.0 * PI / 180.0;
        let aspect = 4.0 / 3.0;
        let znear = 1.0;
        let zfar = 100.0;
        let perspective1 = Mat4::perspective(60.0, 4.0 / 3.0, 1.0, 100.0);
        let perspective2 = nalgebra::base::Matrix4::new_perspective(aspect, fov, znear, zfar);
        let lookat1 = Mat4::lookat(
            Vec3::new(-1.0, -1.0, 5.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        let lookat2 = nalgebra::base::Matrix4::look_at_rh(
            &nalgebra::Point3::new(-1.0, -1.0, 5.0),
            &nalgebra::Point3::new(1.0, 1.0, 0.0),
            &nalgebra::Vector3::new(0.0, 1.0, 0.0),
        );

        println!("{:?}", perspective1 * lookat1);
        println!("{:?}", perspective2 * lookat2);
    }
    #[test]
    fn test_matrix_vec_mul() {
        let vec1 = Vec4::new(0.4, 0.3, 0.5, 1.0);
        let vec2 = nalgebra::Vector4::new(0.4, 0.3, 0.5, 1.0);

        let fov = 60.0 * PI / 180.0;
        let aspect = 4.0 / 3.0;
        let znear = 1.0;
        let zfar = 100.0;
        let perspective1 = Mat4::perspective(60.0, 4.0 / 3.0, 1.0, 100.0);
        let perspective2 = nalgebra::base::Matrix4::new_perspective(aspect, fov, znear, zfar);
        let lookat1 = Mat4::lookat(
            Vec3::new(-1.0, -1.0, 5.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        let lookat2 = nalgebra::base::Matrix4::look_at_rh(
            &nalgebra::Point3::new(-1.0, -1.0, 5.0),
            &nalgebra::Point3::new(1.0, 1.0, 0.0),
            &nalgebra::Vector3::new(0.0, 1.0, 0.0),
        );

        println!("{:?}", perspective1 * lookat1 * vec1);
        println!("{:?}", perspective2 * lookat2 * vec2);
    }
    #[test]
    fn test_quat() {
        let angle = 30.0 * PI / 180.0;
        let quat1 = Quat::from_angle_axis(angle, Vec3::new(-3.0, -4.0, 1.0));
        let axis = Unit::new_normalize(nalgebra::Vector3::new(-3.0, -4.0, 1.0));
        let quat2 = nalgebra::UnitQuaternion::from_axis_angle(&axis, angle);
        println!("{:?}", quat1.to_mat4());
        println!("{:?}", quat2.to_homogeneous());
    }
}
