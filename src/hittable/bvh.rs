use std::{path::Path, sync::Arc};

use anyhow::Context;
use ordered_float::OrderedFloat;

use super::{Aabb, Hittable, HittablePtr, Triangle};
use crate::{HitRecord, MaterialPtr, Random, Vec3};

#[derive(Clone)]
pub enum BvhNode {
    Node {
        left: Box<BvhNode>,
        right: Box<BvhNode>,
        bb: Aabb,
    },
    Leaf(HittablePtr),
}

impl BvhNode {
    pub fn new(
        objects: &mut [HittablePtr],
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
    pub fn load(
        obj_file: impl AsRef<Path>,
        time0: f64,
        time1: f64,
        material: MaterialPtr,
        rng: &mut Random,
    ) -> anyhow::Result<Self> {
        let mut polygons: Vec<HittablePtr> = Vec::new();

        let ob = obj::Obj::load(obj_file)?;
        let ob = ob.data;
        let positions = &ob.position;
        let polys = &ob.objects[0].groups[0].polys;

        for obj::SimplePolygon(poly) in polys {
            if poly.len() != 3 {
                todo!();
            }
            let mut it = poly
                .iter()
                .map(|obj::IndexTuple(i, _, _)| Vec3::from(positions[*i]));
            let p0 = it.next().unwrap();
            let p1 = it.next().unwrap();
            let p2 = it.next().unwrap();
            let triangle = Triangle::new(p0, p1, p2, material.clone());

            polygons.push(Arc::new(triangle));
        }

        Ok(Self::new(&mut polygons, time0, time1, rng).context("No bounding box")?)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64, rng: &mut Random) -> Option<HitRecord> {
        match self {
            BvhNode::Node { left, right, bb } => {
                if !bb.hit(r, t_min, t_max) {
                    return None;
                }
                let rec_l = left.hit(r, t_min, t_max, rng);
                let t_max = rec_l.as_ref().map_or(t_max, |r| r.t);
                let rec_r = right.hit(r, t_min, t_max, rng);
                rec_r.or(rec_l)
            }
            BvhNode::Leaf(h) => h.hit(r, t_min, t_max, rng),
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        match self {
            BvhNode::Node { bb, .. } => Some(bb.clone()),
            BvhNode::Leaf(h) => h.bounding_box(time0, time1),
        }
    }
}
