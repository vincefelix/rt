use crate::vec3::{Point3, Vec3};

pub struct Light {
    pub position: Point3,
    pub intensity: f64,
    pub color: Vec3,
}

impl Light {
    pub fn new(position: Point3, intensity: f64, color: Vec3) -> Self {
        Self {
            position,
            intensity,
            color,
        }
    }

    pub fn direction_to_light(&self, point: &Point3) -> Vec3 {
        (self.position - *point).unit_vector()
    }

    pub fn get_intensity(&self, _distance: f64) -> f64 {
        self.intensity
    }
}
