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
    m: [f32; 16],
}

impl Mat4 {
    pub fn frustum(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Self {
        return Mat4::new([
            (2.0 * n) / (r - l),
            0.0,
            0.0,
            0.0,
            0.0,
            (2.0 * n) / (t - b),
            0.0,
            0.0,
            (r + l) / (r - l),
            (t + b) / (t - b),
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

    pub fn new(m: [f32; 16]) -> Self {
        Mat4 { m }
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        return self.column(0) * rhs.x
            + self.column(1) * rhs.y
            + self.column(2) * rhs.z
            + self.column(3) * rhs.w;
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
