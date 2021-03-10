use std::sync::Arc;

use ordered_float::OrderedFloat;

use super::{Aabb, Hittable};
use crate::Random;

#[derive(Clone)]
pub enum BvhNode {
    Node {
        left: Box<BvhNode>,
        right: Box<BvhNode>,
        bb: Aabb,
    },
    Leaf(Arc<dyn Hittable + Sync + Send>),
}

impl BvhNode {
    pub fn new(
        objects: &mut [Arc<dyn Hittable + Sync + Send>],
        time0: f64,
        time1: f64,
        rng: &mut Random,
    ) -> Option<Self> {
        match objects.len() {
            0 => None,
            1 => Some(BvhNode::Leaf(objects.first().unwrap().clone())),
            len => {
                match rng.range_i32(0, 3) {
                    0 => objects.sort_by_cached_key(|o| {
                        OrderedFloat(o.bounding_box(time0, time1).unwrap().minimum.x)
                    }),
                    1 => objects.sort_by_cached_key(|o| {
                        OrderedFloat(o.bounding_box(time0, time1).unwrap().minimum.y)
                    }),
                    _ => objects.sort_by_cached_key(|o| {
                        OrderedFloat(o.bounding_box(time0, time1).unwrap().minimum.z)
                    }),
                }
                let (l, r) = objects.split_at_mut(len / 2);
                let left = Self::new(l, time0, time1, rng)?;
                let right = Self::new(r, time0, time1, rng)?;
                let bb = left
                    .bounding_box(time0, time1)?
                    .surrounding_box(&right.bounding_box(time0, time1)?);
                Some(BvhNode::Node {
                    left: Box::new(left),
                    right: Box::new(right),
                    bb,
                })
            }
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<crate::HitRecord> {
        match self {
            BvhNode::Node { left, right, bb } => {
                if !bb.hit(r, t_min, t_max) {
                    return None;
                }
                let rec_l = left.hit(r, t_min, t_max);
                let t_max = rec_l.as_ref().map_or(t_max, |r| r.t);
                let rec_r = right.hit(r, t_min, t_max);
                rec_r.or(rec_l)
            }
            BvhNode::Leaf(h) => h.hit(r, t_min, t_max),
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        match self {
            BvhNode::Node { bb, .. } => Some(bb.clone()),
            BvhNode::Leaf(h) => h.bounding_box(time0, time1),
        }
    }
}
