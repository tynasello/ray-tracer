use std::{f64::consts::PI, ops::{Add, Mul, Sub}};

/*

3D Vector

*/

#[derive(Clone)]
pub struct Vec3d {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        if m != 0.0 {
            self * (1.0 / m)
        } else {
            self.clone()
        }
    }

    pub fn reflect(&self, norm: &Self) -> Self {
        let norm = norm.normalize();
        &(&(&norm * (&norm * self)) * 2.0) - self
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for &Vec3d {
    type Output = Vec3d;
    
    fn add(self, b: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        }
    }
}

impl Sub for &Vec3d {
    type Output = Vec3d;

    fn sub(self, b: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        }
    }
}

// Scaling vector
impl Mul<f64> for &Vec3d {
    type Output = Vec3d;

    fn mul(self, f: f64) -> Vec3d {
        Vec3d {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}

// Dot product
impl Mul for &Vec3d {
    type Output = f64;
    
    fn mul(self, b: &Vec3d) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
}

/*

Ray

*/

pub struct Ray {
    origin: Vec3d,
    dir: Vec3d
}

impl Ray {
    pub fn new(origin: Vec3d, dir: Vec3d) -> Self {
        Self {
            origin,
            dir
        }
    }

    pub fn origin(&self) -> &Vec3d {
        &self.origin
    }

    pub fn dir(&self) -> &Vec3d {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Vec3d {
        return &self.origin + &(&self.dir * t);
    }
}

/*

3x3 Matrix

*/

pub struct Mat3 {
    data: [[f64; 3]; 3]
}

impl Mat3 { 
    pub fn new(m: [[f64; 3]; 3]) -> Self {
        Self { data: m }
    }

    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_x(deg: f64) -> Self {
        let angle = deg * (PI / 180.0);
        Self {
            data: [
                [1.0, 0.0, 0.0],
                [0.0,  angle.cos(), -angle.sin()],
                [0.0,  angle.sin(), angle.cos()],
            ],
        }
    }

    pub fn rotation_y(deg: f64) -> Self {
        let angle = deg * (PI / 180.0);
        Self {
            data: [
                [angle.cos(), 0.0, angle.sin()],
                [0.0, 1.0,  0.0],
                [-angle.sin(),  0.0, angle.cos()],
            ],
        }
    }

    pub fn rotation_z(deg: f64) -> Self {
        let angle = deg * (PI / 180.0);
        Self {
            data: [
                [angle.cos(), -angle.sin(), 0.0],
                [angle.sin(),  angle.cos(), 0.0],
                [0.0,  0.0, 1.0],
            ],
        }
    }

    // Rotation about a specified axis
    pub fn rotation_matrix(axis: &Vec3d, angle: f64) -> Self {
        let cos_angle = angle.to_radians().cos();
        let sin_angle = angle.to_radians().sin();

        let u_skew = [
            [0.0, -axis.z, axis.y],
            [axis.z, 0.0, -axis.x],
            [-axis.y, axis.x, 0.0],
        ];

        // Rodrigues' formula: [u]^2 = u * u^T
        let u_skew_squared = [
            [
                axis.x * axis.x + axis.y * axis.y + axis.z * axis.z,
                axis.x * axis.y * (1.0 - cos_angle) - axis.z * sin_angle,
                axis.x * axis.z * (1.0 - cos_angle) + axis.y * sin_angle,
            ],
            [
                axis.x * axis.y * (1.0 - cos_angle) + axis.z * sin_angle,
                axis.x * axis.x + axis.y * axis.y + axis.z * axis.z,
                axis.y * axis.z * (1.0 - cos_angle) - axis.x * sin_angle,
            ],
            [
                axis.x * axis.z * (1.0 - cos_angle) - axis.y * sin_angle,
                axis.y * axis.z * (1.0 - cos_angle) + axis.x * sin_angle,
                axis.x * axis.x + axis.y * axis.y + axis.z * axis.z,
            ],
        ];

        let mut result = Mat3::identity().data;

        // Apply Rodrigues' formula: R = I + sin(angle) * [u] + (1 - cos(angle)) * [u]^2
        for i in 0..3 {
            for j in 0..3 {
                result[i][j] += sin_angle * u_skew[i][j] + (1.0 - cos_angle) * u_skew_squared[i][j];
            }
        }

        Self::new(result)
    }
}

// Matrix multiplication
impl Mul for &Mat3 {
    type Output = Mat3;

    fn mul(self, other: &Mat3) -> Mat3 {
        let mut result = Mat3::identity();
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = self.data[i][0] * other.data[0][j] +
                                    self.data[i][1] * other.data[1][j] +
                                    self.data[i][2] * other.data[2][j];
            }
        }
        result
    }
}

// Vector transformation
impl Mul<&Vec3d> for &Mat3 {
    type Output = Vec3d;

    fn mul(self, v: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.data[0][0] * v.x + self.data[0][1] * v.y + self.data[0][2] * v.z,
            y: self.data[1][0] * v.x + self.data[1][1] * v.y + self.data[1][2] * v.z,
            z: self.data[2][0] * v.x + self.data[2][1] * v.y + self.data[2][2] * v.z,
        }
    }
}