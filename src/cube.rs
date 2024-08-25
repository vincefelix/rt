use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::rc::Rc;
use crate::material::Material;

pub struct Cube {
    pub min: Point3,
    pub max: Point3,
    pub mat: Rc<dyn Material>,
}

impl Cube {
    pub fn new(min: Point3, max: Point3, mat: Rc<dyn Material>) -> Cube {
        Cube { min, max, mat }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64, rec: &mut HitRecord) -> bool {
        for a in 0..3 {
            // Accès explicite aux composantes de Vec3
            let inv_d = if a == 0 {
                1.0 / r.direction().x()
            } else if a == 1 {
                1.0 / r.direction().y()
            } else {
                1.0 / r.direction().z()
            };

            let mut t0 = if a == 0 {
                (self.min.x() - r.origin().x()) * inv_d
            } else if a == 1 {
                (self.min.y() - r.origin().y()) * inv_d
            } else {
                (self.min.z() - r.origin().z()) * inv_d
            };

            let mut t1 = if a == 0 {
                (self.max.x() - r.origin().x()) * inv_d
            } else if a == 1 {
                (self.max.y() - r.origin().y()) * inv_d
            } else {
                (self.max.z() - r.origin().z()) * inv_d
            };

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }

        rec.t = t_min;
        rec.p = r.at(rec.t);

        // Calculer la normale en fonction de la face touchée
        if rec.p.x() <= self.min.x() + 1e-4 {
            rec.normal = Vec3::new(-1.0, 0.0, 0.0);
        } else if rec.p.x() >= self.max.x() - 1e-4 {
            rec.normal = Vec3::new(1.0, 0.0, 0.0);
        } else if rec.p.y() <= self.min.y() + 1e-4 {
            rec.normal = Vec3::new(0.0, -1.0, 0.0);
        } else if rec.p.y() >= self.max.y() - 1e-4 {
            rec.normal = Vec3::new(0.0, 1.0, 0.0);
        } else if rec.p.z() <= self.min.z() + 1e-4 {
            rec.normal = Vec3::new(0.0, 0.0, -1.0);
        } else if rec.p.z() >= self.max.z() - 1e-4 {
            rec.normal = Vec3::new(0.0, 0.0, 1.0);
        }

        rec.set_face_normal(r, rec.normal);
        rec.mat = Some(self.mat.clone());
        true
    }
}
