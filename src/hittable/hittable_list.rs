use std::sync::Arc;

use super::{HitRecord, Hittable};

#[derive(Clone, Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(temp_rec) = object.as_ref().hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec.replace(temp_rec);
            }
        }
        rec
    }
}
