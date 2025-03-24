use std::f64::EPSILON;

use crate::linalg::{Ray, Vec3d};
use crate::utils::Range;

#[derive(Clone)]
pub enum Material {
    // Every surface has a material type that describes its properties

    // Light reflects off a point on a matte object equally in every direction (diffuse reflection)
    // This is possible because at the microscopic level, the material is composed up of irregular surfaces pointing in random directions
    // A point will recieve less light the larger the angle between the ray and normal vector.
    Matte,

    // At the extreme (a perfect mirror), a ray of light intersecting a shiny object will be reflected in one direction (symmetrical to normal vector)
    // This material exhibits specular reflection. A point receives less light the larger the angle between the vector from the point to the camera, and the reflected light ray vector
    // Specular exponent: higher means more shiny, i.e there is less shine as camera moves away from reflected ray
    // Reflection ratio: a ratio between 0 and 1 that describes how reflective the material is, e.g. 0 is not reflective, 1 is a perfect mirror
    Shiny { spclr_exp: f64, refl_rat: f64 }
}

pub trait Object: Send + Sync {
    fn get_color(&self) -> &usize;
    fn get_material(&self) -> &Material;

    // Get the vector that is perpendicular to the object surface and goes through the specified point. 
    // Vector must be of unit length, and should be facing outwards (if possible)
    fn get_normal(&self, p: &Vec3d) -> Option<Vec3d>;

    // Find the closest intersection point of the obj along the ray. Check all points (ray at t) within the t range, and return t
    fn get_closest_intersection(&self, ray: &Ray, t_range: &Range<f64>) -> Option<f64>;
}

pub fn closest_intersection<'a>(objs: &'a [Box<dyn Object>], ray: &Ray, t_range: &Range<f64>) -> Option<(&'a Box<dyn Object>, Vec3d)> {
    // Find and return the closest object along the ray, and it's intersection point with the ray

    let mut closest_t = t_range.max;
    let mut closest_obj: Option<&Box<dyn Object>> = None{};

    for obj in objs {
        let t = obj.get_closest_intersection(ray, t_range);
        if let Some(step) = t {
            if step < closest_t {
                closest_t = step;
                closest_obj = Some(obj);
            }
        }
    }
    
    if let Some(closest_obj) = closest_obj {
        Some((closest_obj, ray.at(closest_t)))
    } else {
        None
    }
}

/*

Sphere

*/

pub struct Sphere {
    center: Vec3d,
    radius: f64,
    color: usize,
    material: Material
}

impl Sphere {
    pub fn new(center: Vec3d, radius: f64, color: usize, material: Material) -> Self {
        Self {
            center,
            radius,
            color,
            material
        }
    }
}

impl Object for Sphere {
    fn get_color(&self) -> &usize {
        &self.color
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_normal(&self, p: &Vec3d) -> Option<Vec3d> {
        Some((p - &self.center).normalize())
    }

    fn get_closest_intersection(&self, ray: &Ray, t_range: &Range<f64>) -> Option<f64> {
        let c_o = ray.origin() - &self.center;

        let a = ray.dir() * ray.dir();
        let b = 2.0 * (&c_o * ray.dir());
        let c = &c_o * &c_o - self.radius * self.radius;

        let discnm: f64 = b * b - 4.0 * a * c;

        if discnm < 0.0 { 
            // No intersections
            return None;
        } else { 
            // 1 or 2 intersections
            let discmn_sqrt = discnm.sqrt();
            let t1 = (-b + discmn_sqrt) / (2.0 * a);
            let t2 = (-b - discmn_sqrt) / (2.0 * a);

            if t1 >= t_range.min && t1 <= t_range.max && t2 >= t_range.min && t2 <= t_range.max {
                if t1 < t2 {
                    return Some(t1);
                } else {
                    return Some(t2);
                }
            } else if t1 >= t_range.min && t1 <= t_range.max {
                return Some(t1);
            } else if t2 >= t_range.min && t2 <= t_range.max {
                return Some(t2);
            } else {
                return None;
            }
        }
    }
}

/*

Triangle

*/

pub struct Triangle {
    ps: [Vec3d; 3],
    color: usize,
    material: Material
}

impl Triangle {
    pub fn new(ps: [Vec3d; 3], color: usize, material: Material) -> Self {
        Self {
            ps,
            color,
            material
        }
    }
}

impl Object for Triangle {
    fn get_color(&self) -> &usize {
        &self.color
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_normal(&self, p: &Vec3d) -> Option<Vec3d> {
        let e1 = &self.ps[1] - &self.ps[0];
        let e2 = &self.ps[2] - &self.ps[0];
        let normal = e1.cross(&e2).normalize();

        let ray = Ray::new(p.clone(), normal.clone());
    
        if let Some(_) = self.get_closest_intersection(&ray, &Range{min: -EPSILON * 1000000.0, max: EPSILON * 1000000.0}) {
            Some(normal)
        } else {
            None
        }
    }

    fn get_closest_intersection(&self, ray: &Ray, t_range: &Range<f64>) -> Option<f64> {
        // Möller–Trumbore ray-triangle intersection algorithm

        let e1 = &self.ps[1] - &self.ps[0];
        let e2 = &self.ps[2] - &self.ps[0];
    
        let v_cross_e2 = ray.dir().cross(&e2);
        let det = &e1 * &v_cross_e2;
    
        if det > -EPSILON && det < EPSILON {
            return None;
        }
    
        let inv_det = 1.0 / det;
        let s = ray.origin() - &self.ps[0];
        let u = inv_det * (&s * &v_cross_e2);
        if u < 0.0 || u > 1.0 {
            return None;
        }
    
    	let s_cross_e1 = s.cross(&e1);
        let a = inv_det * (ray.dir() * &s_cross_e1);
        if a < 0.0 || u + a > 1.0 {
            return None;
        }

        let t = inv_det * (&e2 * &s_cross_e1);
    
        if t_range.min <= t && t <= t_range.max {
            return Some(t);
        }
        else {
            return None;
        }
    }
}

pub struct RectangularPrism {
    ts: Vec<Triangle>,
    color: usize,
    material: Material,
}

impl RectangularPrism {
    pub fn new(origin: Vec3d, width: f64, height: f64, depth: f64, color: usize, material: Material) -> Self {
        let mut ts = Vec::new();
        let p0 = origin.clone();
        let p1 = &origin + &Vec3d::new(width, 0.0, 0.0);
        let p2 = &origin + &Vec3d::new(width, height, 0.0);
        let p3 = &origin + &Vec3d::new(0.0, height, 0.0);
        let p4 = &origin + &Vec3d::new(0.0, 0.0, depth);
        let p5 = &origin + &Vec3d::new(width, 0.0, depth);
        let p6 = &origin + &Vec3d::new(width, height, depth);
        let p7 = &origin + &Vec3d::new(0.0, height, depth);

        let faces = vec![
            (&p0, &p1, &p2, &p3), // Front
            (&p4, &p5, &p6, &p7), // Back
            (&p0, &p1, &p5, &p4), // Bottom
            (&p3, &p2, &p6, &p7), // Top
            (&p0, &p3, &p7, &p4), // Left
            (&p1, &p2, &p6, &p5), // Right
        ];

        for (a, b, c, d) in faces {
            ts.push(Triangle::new([a.clone(), b.clone(), c.clone()], color, material.clone()));
            ts.push(Triangle::new([a.clone(), c.clone(), d.clone()], color, material.clone()));
        }

        Self { 
            color, 
            material, 
            ts 
        }
    }
}

impl Object for RectangularPrism {
    fn get_color(&self) -> &usize {
        &self.color
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_normal(&self, p: &Vec3d) -> Option<Vec3d> {
        for tri in &self.ts {
            if let Some(normal) = tri.get_normal(p) {
                return Some(normal);
            }
        }
        None
    }

    fn get_closest_intersection(&self, ray: &Ray, t_range: &Range<f64>) -> Option<f64> {
        let mut closest_t: Option<f64> = None;
        for tri in &self.ts {
            if let Some(t) = tri.get_closest_intersection(ray, t_range) {
                closest_t = match closest_t {
                    Some(closest_t) if t < closest_t => Some(t),
                    Some(_) => closest_t,
                    None => Some(t),
                };
            }
        }
        closest_t
    }
}