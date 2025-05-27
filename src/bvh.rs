use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

/// A Bounding Volume Hierarchy (BVH) acceleration structure for ray tracing.
/// This structure organizes objects in a binary tree to accelerate ray-object intersection tests.
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

/// A node in the BVH tree structure. Can be either a branch (containing two child nodes)
/// or a leaf (containing a single hittable object).
pub struct Bvh {
    tree: BvhNode,
    bbox: Aabb,
}

#[derive(Debug)]
pub enum BvhError {
    MissingBoundingBox,
    EmptyObjectList,
}

impl fmt::Display for BvhError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BvhError::MissingBoundingBox => write!(f, "Object has no bounding box"),
            BvhError::EmptyObjectList => write!(f, "Cannot create BVH from empty object list"),
        }
    }
}

impl Error for BvhError {}

impl Bvh {
    /// Creates a new BVH from a list of hittable objects.
    /// The objects are organized into a binary tree structure for efficient ray intersection tests.
    pub fn new(mut objects: Vec<Box<dyn Hittable>>) -> Result<Self, BvhError> {
        if objects.is_empty() {
            return Err(BvhError::EmptyObjectList);
        }
        let tree = Bvh::build(&mut objects)?;
        let bbox = tree.bounding_box().ok_or(BvhError::MissingBoundingBox)?;
        Ok(Self { tree, bbox })
    }

    fn build(objects: &mut [Box<dyn Hittable>]) -> Result<BvhNode, BvhError> {
        let len = objects.len();
        if len == 0 {
            return Err(BvhError::EmptyObjectList);
        }

        // Find the axis with the largest spread
        let mut min_bounds = [f64::INFINITY; 3];
        let mut max_bounds = [f64::NEG_INFINITY; 3];

        for obj in objects.iter() {
            let bbox = obj
                .bounding_box(0.0, 1.0)
                .ok_or(BvhError::MissingBoundingBox)?;
            for axis in 0..3 {
                let interval = bbox.axis_interval(axis);
                min_bounds[axis] = min_bounds[axis].min(interval.min());
                max_bounds[axis] = max_bounds[axis].max(interval.max());
            }
        }

        let axis = (0..3)
            .max_by(|&a, &b| {
                let spread_a = max_bounds[a] - min_bounds[a];
                let spread_b = max_bounds[b] - min_bounds[b];
                spread_a.partial_cmp(&spread_b).unwrap_or(Ordering::Equal)
            })
            .unwrap_or(0);

        let comparator = |a: &dyn Hittable, b: &dyn Hittable| -> Result<Ordering, BvhError> {
            let box_a = a
                .bounding_box(0.0, 1.0)
                .ok_or(BvhError::MissingBoundingBox)?;
            let box_b = b
                .bounding_box(0.0, 1.0)
                .ok_or(BvhError::MissingBoundingBox)?;
            Ok(box_a
                .axis_interval(axis)
                .min()
                .partial_cmp(&box_b.axis_interval(axis).min())
                .unwrap_or(Ordering::Equal))
        };

        match len {
            1 => {
                let bbox = objects[0]
                    .bounding_box(0.0, 1.0)
                    .ok_or(BvhError::MissingBoundingBox)?;
                Ok(BvhNode::Leaf {
                    object: std::mem::replace(&mut objects[0], Box::new(DummyHittable)),
                    bbox,
                })
            }
            2 => {
                let mut objs = vec![
                    std::mem::replace(&mut objects[0], Box::new(DummyHittable)),
                    std::mem::replace(&mut objects[1], Box::new(DummyHittable)),
                ];
                objs.sort_by(|a, b| comparator(a.as_ref(), b.as_ref()).unwrap_or(Ordering::Equal));
                let left = Bvh::build(&mut [objs.remove(0)])?;
                let right = Bvh::build(&mut [objs.remove(0)])?;
                let bbox = Aabb::surrounding(
                    &left.bounding_box().ok_or(BvhError::MissingBoundingBox)?,
                    &right.bounding_box().ok_or(BvhError::MissingBoundingBox)?,
                );
                Ok(BvhNode::Branch {
                    left: Box::new(left),
                    right: Box::new(right),
                    bbox,
                })
            }
            _ => {
                objects
                    .sort_by(|a, b| comparator(a.as_ref(), b.as_ref()).unwrap_or(Ordering::Equal));
                let mid = len / 2;
                let (left_objs, right_objs) = objects.split_at_mut(mid);
                let left = Bvh::build(left_objs)?;
                let right = Bvh::build(right_objs)?;
                let bbox = Aabb::surrounding(
                    &left.bounding_box().ok_or(BvhError::MissingBoundingBox)?,
                    &right.bounding_box().ok_or(BvhError::MissingBoundingBox)?,
                );
                Ok(BvhNode::Branch {
                    left: Box::new(left),
                    right: Box::new(right),
                    bbox,
                })
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
    use crate::sphere::SphereBuilder;
    use crate::texture::{SolidColor, TextureEnum};
    use crate::vec3::Vec3;

    fn test_material() -> Material {
        Lambertian::new(Box::new(TextureEnum::SolidColor(SolidColor::new(
            Color::new(0.8, 0.3, 0.3),
        ))))
    }

    #[test]
    fn test_bvh_construction_and_bbox() {
        let s1 = SphereBuilder::new()
            .center(Point3::new(0.0, 0.0, -1.0))
            .radius(0.5)
            .material(test_material())
            .build()
            .unwrap();
        let s2 = SphereBuilder::new()
            .center(Point3::new(0.0, -100.5, -1.0))
            .radius(100.0)
            .material(test_material())
            .build()
            .unwrap();
        let objects: Vec<Box<dyn Hittable>> = vec![Box::new(s1), Box::new(s2)];
        let bvh = Bvh::new(objects).unwrap();
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
        // let s1: Box<dyn Hittable> = Box::new(Sphere::new(
        //     Point3::new(0.0, 0.0, -1.0),
        //     0.5,
        //     test_material(),
        // ));
        // let s2: Box<dyn Hittable> = Box::new(Sphere::new(
        //     Point3::new(0.0, -100.5, -1.0),
        //     100.0,
        //     test_material(),
        // ));
        let s1 = SphereBuilder::new()
            .center(Point3::new(0.0, 0.0, -1.0))
            .radius(0.5)
            .material(test_material())
            .build()
            .unwrap();
        let s2 = SphereBuilder::new()
            .center(Point3::new(0.0, -100.5, -1.0))
            .radius(100.0)
            .material(test_material())
            .build()
            .unwrap();
        let objects: Vec<Box<dyn Hittable>> = vec![Box::new(s1), Box::new(s2)];
        let bvh = Bvh::new(objects).unwrap();
        // Ray that misses everything
        let ray = Ray::new(Point3::new(2.0, 2.0, 0.0), Vec3::new(0.0, 0.0, -1.0), 0.0);
        let interval = Interval::new(0.001, f64::INFINITY);
        assert!(bvh.hit(&ray, interval).is_none());
    }

    #[test]
    fn test_bvh_hit_detect() {
        let s1 = SphereBuilder::new()
            .center(Point3::new(0.0, 0.0, -1.0))
            .radius(0.5)
            .material(test_material())
            .build()
            .unwrap();
        let s2 = SphereBuilder::new()
            .center(Point3::new(0.0, -100.5, -1.0))
            .radius(100.0)
            .material(test_material())
            .build()
            .unwrap();
        let objects: Vec<Box<dyn Hittable>> = vec![Box::new(s1), Box::new(s2)];
        let bvh = Bvh::new(objects).unwrap();
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
        let s1 = SphereBuilder::new()
            .center(Point3::new(1.0, 2.0, 3.0))
            .radius(1.0)
            .material(test_material())
            .build()
            .unwrap();
        let objects: Vec<Box<dyn Hittable>> = vec![Box::new(s1)];
        let bvh = Bvh::new(objects).unwrap();
        let bbox = bvh.bounding_box(0.0, 1.0).unwrap();
        let min_x = bbox.axis_interval(0).min();
        let max_x = bbox.axis_interval(0).max();
        assert!((min_x - 0.0).abs() < 1e-6);
        assert!((max_x - 2.0).abs() < 1e-6);
    }
}
