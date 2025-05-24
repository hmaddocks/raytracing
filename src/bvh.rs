use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use rand::Rng;
use std::cmp::Ordering;

pub enum BvhNode {
    Branch {
        left: Box<BvhNode>,
        right: Box<BvhNode>,
        bbox: Aabb,
    },
    Leaf {
        object: Box<dyn Hittable>,
        bbox: Aabb,
    },
}

pub struct Bvh {
    tree: BvhNode,
    bbox: Aabb,
}

impl Bvh {
    pub fn new(mut objects: Vec<Box<dyn Hittable>>) -> Self {
        let tree = Bvh::build(&mut objects);
        let bbox = tree.bounding_box().unwrap();
        Self { tree, bbox }
    }

    fn build(objects: &mut [Box<dyn Hittable>]) -> BvhNode {
        let len = objects.len();
        let axis = rand::rng().random_range(0..3);
        let comparator = |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| {
            let box_a = a
                .bounding_box(0.0, 1.0)
                .expect("No bounding box in BVH node.");
            let box_b = b
                .bounding_box(0.0, 1.0)
                .expect("No bounding box in BVH node.");
            box_a
                .axis_interval(axis)
                .min()
                .partial_cmp(&box_b.axis_interval(axis).min())
                .unwrap_or(Ordering::Equal)
        };
        match len {
            1 => {
                let bbox = objects[0].bounding_box(0.0, 1.0).unwrap();
                BvhNode::Leaf {
                    object: std::mem::replace(&mut objects[0], Box::new(DummyHittable)),
                    bbox,
                }
            }
            2 => {
                let mut objs = vec![
                    std::mem::replace(&mut objects[0], Box::new(DummyHittable)),
                    std::mem::replace(&mut objects[1], Box::new(DummyHittable)),
                ];
                objs.sort_by(comparator);
                let left = Bvh::build(&mut [objs.remove(0)]);
                let right = Bvh::build(&mut [objs.remove(0)]);
                let bbox = Aabb::surrounding(
                    &left.bounding_box().unwrap(),
                    &right.bounding_box().unwrap(),
                );
                BvhNode::Branch {
                    left: Box::new(left),
                    right: Box::new(right),
                    bbox,
                }
            }
            _ => {
                objects.sort_by(comparator);
                let mid = len / 2;
                let (left_objs, right_objs) = objects.split_at_mut(mid);
                let left = Bvh::build(left_objs);
                let right = Bvh::build(right_objs);
                let bbox = Aabb::surrounding(
                    &left.bounding_box().unwrap(),
                    &right.bounding_box().unwrap(),
                );
                BvhNode::Branch {
                    left: Box::new(left),
                    right: Box::new(right),
                    bbox,
                }
            }
        }
    }
}

impl Hittable for Bvh {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        self.tree.hit(r, ray_t)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bbox)
    }
}

impl BvhNode {
    pub fn bounding_box(&self) -> Option<Aabb> {
        match self {
            BvhNode::Branch { bbox, .. } => Some(*bbox),
            BvhNode::Leaf { bbox, .. } => Some(*bbox),
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            BvhNode::Branch { left, right, bbox } => {
                bbox.hit(r, ray_t)?;
                let hit_left = left.hit(r, ray_t);
                let t_max = if let Some(ref rec) = hit_left {
                    Interval::new(ray_t.min(), rec.t)
                } else {
                    ray_t
                };
                let hit_right = right.hit(r, t_max);
                hit_right.or(hit_left)
            }
            BvhNode::Leaf { object, bbox } => {
                bbox.hit(r, ray_t)?;
                object.hit(r, ray_t)
            }
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bounding_box()
    }
}

struct DummyHittable;
impl Hittable for DummyHittable {
    fn hit(&self, _r: &Ray, _ray_t: Interval) -> Option<HitRecord> {
        None
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::hittable::Hittable;
    use crate::interval::Interval;
    use crate::material::{Lambertian, Material};
    use crate::point3::Point3;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::texture::SolidColor;
    use crate::vec3::Vec3;

    fn test_material() -> Material {
        Lambertian::new(Box::new(SolidColor::new(Color::new(0.8, 0.3, 0.3))))
    }

    #[test]
    fn test_bvh_construction_and_bbox() {
        let s1: Box<dyn Hittable> = Box::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            test_material(),
        ));
        let s2: Box<dyn Hittable> = Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            test_material(),
        ));
        let objects: Vec<Box<dyn Hittable>> = vec![s1, s2];
        let bvh = Bvh::new(objects);
        let bbox = bvh.bounding_box(0.0, 1.0).unwrap();
        // The bounding box should enclose both spheres (rough check)
        let min_x = bbox.axis_interval(0).min();
        let max_x = bbox.axis_interval(0).max();
        let min_y = bbox.axis_interval(1).min();
        let max_y = bbox.axis_interval(1).max();
        let min_z = bbox.axis_interval(2).min();
        let max_z = bbox.axis_interval(2).max();
        println!("min_x: {}, max_x: {}", min_x, max_x);
        println!("min_y: {}, max_y: {}", min_y, max_y);
        println!("min_z: {}, max_z: {}", min_z, max_z);
        assert!(min_x <= -100.0 && max_x >= 100.0);
        assert!(min_y <= -100.5 && max_y >= 0.5);
        assert!(min_z <= -101.0 && max_z >= 0.0);
    }

    #[test]
    fn test_bvh_hit_miss() {
        let s1: Box<dyn Hittable> = Box::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            test_material(),
        ));
        let s2: Box<dyn Hittable> = Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            test_material(),
        ));
        let objects: Vec<Box<dyn Hittable>> = vec![s1, s2];
        let bvh = Bvh::new(objects);
        // Ray that misses everything
        let ray = Ray::new(Point3::new(2.0, 2.0, 0.0), Vec3::new(0.0, 0.0, -1.0), 0.0);
        let interval = Interval::new(0.001, f64::INFINITY);
        assert!(bvh.hit(&ray, interval).is_none());
    }

    #[test]
    fn test_bvh_hit_detect() {
        let s1: Box<dyn Hittable> = Box::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            test_material(),
        ));
        let s2: Box<dyn Hittable> = Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            test_material(),
        ));
        let objects: Vec<Box<dyn Hittable>> = vec![s1, s2];
        let bvh = Bvh::new(objects);
        // Ray that hits the small sphere
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), 0.0);
        let interval = Interval::new(0.001, f64::INFINITY);
        let hit = bvh.hit(&ray, interval);
        assert!(hit.is_some());
        let rec = hit.unwrap();
        // The hit point should be close to the small sphere's center (z ~ -1.0)
        assert!((rec.position.z() + 1.0).abs() < 0.6);
    }

    #[test]
    fn test_bvh_empty_and_single() {
        // Empty BVH (should not panic, but not useful)
        // let objects: Vec<Box<dyn Hittable>> = vec![];
        // let bvh = Bvh::new(objects); // Would panic on unwrap

        // Single object BVH
        let s1: Box<dyn Hittable> = Box::new(Sphere::new(
            Point3::new(1.0, 2.0, 3.0),
            1.0,
            test_material(),
        ));
        let objects: Vec<Box<dyn Hittable>> = vec![s1];
        let bvh = Bvh::new(objects);
        let bbox = bvh.bounding_box(0.0, 1.0).unwrap();
        let min_x = bbox.axis_interval(0).min();
        let max_x = bbox.axis_interval(0).max();
        assert!((min_x - 0.0).abs() < 1e-6);
        assert!((max_x - 2.0).abs() < 1e-6);
    }
}
