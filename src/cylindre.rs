use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use std::rc::Rc;
use crate::material::Material;

pub struct Cylinder {
    pub base: Point3,  // Le centre de la base inférieure du cylindre
    pub axis: Vec3,    // Le vecteur axe du cylindre (direction et longueur)
    pub radius: f64,   // Rayon du cylindre
    pub height: f64,   // Hauteur du cylindre
    pub mat: Rc<dyn Material>,
}

impl Cylinder {
    pub fn new(base: Point3, axis: Vec3, radius: f64, height: f64, mat: Rc<dyn Material>) -> Cylinder {
        Cylinder { base, axis, radius, height, mat }
    }
}

// impl Hittable for Cylinder {
//     fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
//         // Calculer les paramètres de la surface latérale
//         let oc = r.origin() - self.base;  // Vecteur de la base du cylindre au point d'origine du rayon
//         let dir = r.direction();  // Direction du rayon

//         let a = dir.x() * dir.x() + dir.z() * dir.z();  // Composantes x et z pour la surface latérale
//         let b = 2.0 * (oc.x() * dir.x() + oc.z() * dir.z());
//         let c = oc.x() * oc.x() + oc.z() * oc.z() - self.radius * self.radius;

//         // Discriminant de l'équation quadratique
//         let discriminant = b * b - 4.0 * a * c;

//         if discriminant < 0.0 {
//             return false;
//         }

//         // Résoudre pour t1 et t2 (points d'intersection possibles)
//         let sqrt_discriminant = discriminant.sqrt();
//         let mut t = (-b - sqrt_discriminant) / (2.0 * a);
//         if t < t_min || t > t_max {
//             t = (-b + sqrt_discriminant) / (2.0 * a);
//             if t < t_min || t > t_max {
//                 return false;
//             }
//         }

//         // Vérifier que l'intersection est dans les limites du cylindre (hauteur)
//         let y = r.origin().y() + t * r.direction().y();  // Coordonnée y du point d'intersection
//         if y < self.base.y() || y > self.base.y() + self.height {
//             return false;
//         }

//         // Enregistrer le point d'intersection sur la surface latérale
//         rec.t = t;
//         rec.p = r.at(rec.t);
//         let outward_normal = Vec3::new(rec.p.x() - self.base.x(), 0.0, rec.p.z() - self.base.z()).unit_vector();
//         rec.set_face_normal(r, outward_normal);
//         rec.mat = Some(self.mat.clone());
        
//         // Vérifier les bases du cylindre (caps)
//         // Base inférieure
//         let t_base1 = (self.base.y() - r.origin().y()) / r.direction().y();
//         if t_base1 >= t_min && t_base1 <= t_max {
//             let p_base1 = r.at(t_base1);
//             if (p_base1.x() - self.base.x()).powi(2) + (p_base1.z() - self.base.z()).powi(2) <= self.radius * self.radius {
//                 rec.t = t_base1;
//                 rec.p = p_base1;
//                 rec.set_face_normal(r, Vec3::new(0.0, -1.0, 0.0));
//                 rec.mat = Some(self.mat.clone());
//                 return true;
//             }
//         }

//         // Base supérieure
//         let t_base2 = (self.base.y() + self.height - r.origin().y()) / r.direction().y();
//         if t_base2 >= t_min && t_base2 <= t_max {
//             let p_base2 = r.at(t_base2);
//             if (p_base2.x() - self.base.x()).powi(2) + (p_base2.z() - self.base.z()).powi(2) <= self.radius * self.radius {
//                 rec.t = t_base2;
//                 rec.p = p_base2;
//                 rec.set_face_normal(r, Vec3::new(0.0, 1.0, 0.0));
//                 rec.mat = Some(self.mat.clone());
//                 return true;
//             }
//         }

//         true
//     }
// }


impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.base;
        let dir = r.direction();

        // Surface latérale du cylindre
        let a = dir.x() * dir.x() + dir.z() * dir.z();
        let b = 2.0 * (oc.x() * dir.x() + oc.z() * dir.z());
        let c = oc.x() * oc.x() + oc.z() * oc.z() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let mut t = (-b - sqrt_discriminant) / (2.0 * a);
            if t < t_min || t > t_max {
                t = (-b + sqrt_discriminant) / (2.0 * a);
            }

            if t >= t_min && t <= t_max {
                let y = r.origin().y() + t * r.direction().y();
                if y >= self.base.y() && y <= self.base.y() + self.height {
                    rec.t = t;
                    rec.p = r.at(rec.t);
                    let outward_normal = Vec3::new(rec.p.x() - self.base.x(), 0.0, rec.p.z() - self.base.z()).unit_vector();
                    rec.set_face_normal(r, outward_normal);
                    rec.mat = Some(self.mat.clone());
                    return true;
                }
            }
        }

        // Vérification de la base inférieure
        let t_base1 = (self.base.y() - r.origin().y()) / r.direction().y();
        if t_base1 >= t_min && t_base1 <= t_max {
            let p_base1 = r.at(t_base1);
            if (p_base1.x() - self.base.x()).powi(2) + (p_base1.z() - self.base.z()).powi(2) <= self.radius * self.radius {
                rec.t = t_base1;
                rec.p = p_base1;
                rec.set_face_normal(r, Vec3::new(0.0, -1.0, 0.0));
                rec.mat = Some(self.mat.clone());
                return true;
            }
        }

        // Vérification de la base supérieure
        let t_base2 = (self.base.y() + self.height - r.origin().y()) / r.direction().y();
        if t_base2 >= t_min && t_base2 <= t_max {
            let p_base2 = r.at(t_base2);
            if (p_base2.x() - self.base.x()).powi(2) + (p_base2.z() - self.base.z()).powi(2) <= self.radius * self.radius {
                rec.t = t_base2;
                rec.p = p_base2;
                rec.set_face_normal(r, Vec3::new(0.0, 1.0, 0.0));
                rec.mat = Some(self.mat.clone());
                return true;
            }
        }

        false
    }
}
