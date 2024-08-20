// rectangle.rs

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use std::rc::Rc;

pub struct Rectangle {
    pub corner: Point3, // Un coin du rectangle
    pub size_x: Vec3,   // Taille en direction x
    pub size_z: Vec3,   // Taille en direction z
    pub mat: Rc<dyn Material>,
}

impl Rectangle {
    pub fn new(corner: Point3, size_x: Vec3, size_z: Vec3, mat: Rc<dyn Material>) -> Rectangle {
        Rectangle {
            corner,
            size_x,
            size_z,
            mat,
        }
    }
}


impl Hittable for Rectangle {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Calcul de la normale basée sur l'orientation dans le monde
        let normal = self.size_x.cross(&self.size_z).unit_vector();

        let denom = dot(normal, r.direction());
        if denom.abs() > 1e-6 {
            let t = dot(self.corner - r.origin(), normal) / denom;
            if t >= t_min && t <= t_max {
                let p = r.at(t);
                let v = p - self.corner;

                // Projeter sur les axes size_x et size_z
                let u = dot(v, self.size_x.unit_vector());
                let w = dot(v, self.size_z.unit_vector());

                if u >= 0.0 && u <= self.size_x.length() && w >= 0.0 && w <= self.size_z.length() {
                    rec.t = t;
                    rec.p = p;
                    rec.normal = normal;
                    rec.mat = Some(self.mat.clone());
                    rec.set_face_normal(r, normal);
                    return true;
                }
            }
        }
        false
    }
}


// impl Hittable for Rectangle {
//     fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
//         // Calculer la normale du rectangle
//         // Utilisation correcte du produit vectoriel
//         let normal = self.size_x.cross(&self.size_z).unit_vector();

//         let denom = dot(normal, r.direction());
//         if denom.abs() > 1e-6 {
//             let t = dot(self.corner - r.origin(), normal) / denom;
//             if t >= t_min && t <= t_max {
//                 let p = r.at(t);
//                 let v = p - self.corner;
//                 let u = dot(v, self.size_x);
//                 let w = dot(v, self.size_z);

//                 if u >= 0.0 && u <= self.size_x.length() && w >= 0.0 && w <= self.size_z.length() {
//                     rec.t = t;
//                     rec.p = p;
//                     rec.normal = normal;
//                     rec.mat = Some(self.mat.clone());
//                     rec.set_face_normal(r, normal);
//                     return true;
//                 }
//             }
//         }
//         false
//     }
// }
