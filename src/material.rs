use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Test(TestMaterial),
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Color, Ray) {
        match self {
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Metal(m) => m.scatter(ray, hit_record),
            Material::Test(t) => t.scatter(ray, hit_record),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }

    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> (Color, Ray) {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let scatter = Ray::new(hit_record.p, scatter_direction);
        (self.albedo, scatter)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Material {
        let fuzz = fuzz.clamp(0.0, 1.0); // Ensure fuzz is between 0 and 1
        Material::Metal(Metal { albedo, fuzz })
    }

    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Color, Ray) {
        // let reflected = hit_record.normal.reflect(&ray.direction());
        let mut reflected = ray.direction().reflect(&hit_record.normal);
        reflected = reflected.unit() + (Vec3::random_unit() * self.fuzz);
        let scatter = Ray::new(hit_record.p, reflected);
        (self.albedo, scatter)
    }
}

// A simple material for testing purposes
#[derive(Clone, Debug, PartialEq)]
pub struct TestMaterial;

impl TestMaterial {
    pub fn new() -> Material {
        Material::Test(TestMaterial)
    }

    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> (Color, Ray) {
        // Simple implementation that just returns white and a ray in the normal direction
        let scatter_direction = hit_record.normal;
        let scatter = Ray::new(hit_record.p, scatter_direction);
        (Color::new(1.0, 1.0, 1.0), scatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point3::Point3;

    // Helper function to create a HitRecord for testing
    fn create_hit_record(p: Point3, normal: Vec3, material: Option<Material>) -> HitRecord {
        let hit_record = HitRecord {
            p,
            normal,
            t: 1.0,
            front_face: true,
            material,
        };
        hit_record
    }

    #[test]
    fn test_lambertian_creation() {
        let albedo = Color::new(0.5, 0.5, 0.5);
        let material = Lambertian::new(albedo);

        match material {
            Material::Lambertian(l) => {
                // Check that the albedo was stored correctly
                assert_eq!(l.albedo, albedo);
            }
            _ => panic!("Expected Lambertian material"),
        }
    }

    #[test]
    fn test_lambertian_scatter() {
        let albedo = Color::new(0.5, 0.5, 0.5);
        let material = Lambertian::new(albedo);

        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let hit_point = Point3::new(0.0, 0.0, 1.0);
        let normal = Vec3::new(0.0, 0.0, -1.0); // Surface normal pointing back

        let hit_record = create_hit_record(hit_point, normal, Some(material.clone()));

        let (scattered_color, scattered_ray) = match material {
            Material::Lambertian(l) => l.scatter(&ray, &hit_record),
            _ => panic!("Expected Lambertian material"),
        };

        // Check that the scattered color is the albedo
        assert_eq!(scattered_color, albedo);

        // Check that the scattered ray originates from the hit point
        assert_eq!(*scattered_ray.origin(), hit_point);

        // In the Lambertian scatter implementation, the scatter direction is:
        // hit_record.normal + Vec3::random_unit()
        // This means the scattered ray will be in the same hemisphere as the normal
        // (dot product with normal should be positive)
        //
        // The normal is pointing in the negative z direction, so the scattered ray
        // should also have a negative z component (pointing away from the origin)
        let dot_product = scattered_ray.direction().dot(&normal);
        assert!(
            dot_product > 0.0,
            "Expected dot product > 0.0, got: {}",
            dot_product
        );
    }

    #[test]
    fn test_metal_creation() {
        let albedo = Color::new(0.8, 0.8, 0.8);

        // Test with fuzz in valid range
        let material1 = Metal::new(albedo, 0.5);
        match material1 {
            Material::Metal(m) => {
                assert_eq!(m.albedo, albedo);
                assert_eq!(m.fuzz, 0.5);
            }
            _ => panic!("Expected Metal material"),
        }

        // Test with fuzz > 1.0 (should be clamped to 1.0)
        let material2 = Metal::new(albedo, 1.5);
        match material2 {
            Material::Metal(m) => {
                assert_eq!(m.albedo, albedo);
                assert_eq!(m.fuzz, 1.0); // Should be clamped to 1.0
            }
            _ => panic!("Expected Metal material"),
        }

        // Test with negative fuzz (should be clamped to 0.0)
        let material3 = Metal::new(albedo, -0.5);
        match material3 {
            Material::Metal(m) => {
                assert_eq!(m.albedo, albedo);
                assert_eq!(m.fuzz, 0.0); // Should be clamped to 0.0
            }
            _ => panic!("Expected Metal material"),
        }
    }

    #[test]
    fn test_metal_scatter() {
        let albedo = Color::new(0.8, 0.8, 0.8);
        let material = Metal::new(albedo, 0.0); // No fuzz for predictable reflection

        // Create a ray coming in at 45 degrees
        let ray_dir = Vec3::new(1.0, -1.0, 0.0).unit();
        let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), ray_dir);

        // Hit point is where the ray intersects the xz-plane
        let hit_point = Point3::new(1.0, 0.0, 0.0);
        let normal = Vec3::new(0.0, 1.0, 0.0); // Normal points straight up

        let hit_record = create_hit_record(hit_point, normal, Some(material.clone()));

        let (scattered_color, scattered_ray) = match material {
            Material::Metal(m) => m.scatter(&ray, &hit_record),
            _ => panic!("Expected Metal material"),
        };

        // Check that the scattered color is the albedo
        assert_eq!(scattered_color, albedo);

        // Check that the scattered ray originates from the hit point
        assert_eq!(*scattered_ray.origin(), hit_point);

        // In the Metal implementation, reflection is calculated using ray.direction().reflect(&hit_record.normal)
        // and then normalized before adding fuzz
        let expected_direction = ray.direction().reflect(&normal).unit();

        // Allow for some floating-point imprecision
        let dir_diff = (*scattered_ray.direction() - expected_direction).length();
        assert!(
            dir_diff < 1e-10,
            "Expected direction: {:?}, got: {:?}",
            expected_direction,
            scattered_ray.direction()
        );
    }

    #[test]
    fn test_metal_scatter_with_fuzz() {
        let albedo = Color::new(0.8, 0.8, 0.8);
        let material = Metal::new(albedo, 1.0); // Maximum fuzz

        // Create a ray coming in at 45 degrees
        let ray_dir = Vec3::new(1.0, -1.0, 0.0).unit();
        let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), ray_dir);

        // Hit point is where the ray intersects the xz-plane
        let hit_point = Point3::new(1.0, 0.0, 0.0);
        let normal = Vec3::new(0.0, 1.0, 0.0); // Normal points straight up

        let hit_record = create_hit_record(hit_point, normal, Some(material.clone()));

        let (scattered_color, scattered_ray) = match material {
            Material::Metal(m) => m.scatter(&ray, &hit_record),
            _ => panic!("Expected Metal material"),
        };

        // Check that the scattered color is the albedo
        assert_eq!(scattered_color, albedo);

        // Check that the scattered ray originates from the hit point
        assert_eq!(*scattered_ray.origin(), hit_point);

        // With maximum fuzz (1.0), the implementation does:
        // reflected = ray.direction().reflect(&hit_record.normal).unit() + (Vec3::random_unit() * 1.0)
        // This means the direction will be the normalized reflection plus a random unit vector
        // Since there's randomness involved, we can't predict the exact direction
        // Instead, we'll just verify that the direction is not zero and has a reasonable length
        let direction_length = scattered_ray.direction().length();
        assert!(
            direction_length > 0.5 && direction_length < 2.5,
            "Expected direction length between 0.5 and 2.5, got: {}",
            direction_length
        );
    }

    #[test]
    fn test_test_material_creation() {
        let material = TestMaterial::new();
        match material {
            Material::Test(_) => {} // Success if it's a TestMaterial
            _ => panic!("Expected TestMaterial"),
        }
    }

    #[test]
    fn test_test_material_scatter() {
        let material = TestMaterial::new();

        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let hit_point = Point3::new(0.0, 0.0, 1.0);
        let normal = Vec3::new(0.0, 0.0, -1.0);

        let hit_record = create_hit_record(hit_point, normal, Some(material.clone()));

        let (scattered_color, scattered_ray) = match material {
            Material::Test(t) => t.scatter(&ray, &hit_record),
            _ => panic!("Expected TestMaterial"),
        };

        // Check that the scattered color is white
        assert_eq!(scattered_color, Color::new(1.0, 1.0, 1.0));

        // Check that the scattered ray originates from the hit point
        assert_eq!(*scattered_ray.origin(), hit_point);

        // Check that the scattered ray direction is the normal
        assert_eq!(*scattered_ray.direction(), normal);
    }

    #[test]
    fn test_material_enum_scatter() {
        // Test that the Material enum correctly delegates to the appropriate implementation

        let albedo = Color::new(0.5, 0.5, 0.5);
        let lambertian = Lambertian::new(albedo);

        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let hit_point = Point3::new(0.0, 0.0, 1.0);
        let normal = Vec3::new(0.0, 0.0, -1.0);

        let hit_record = create_hit_record(hit_point, normal, Some(lambertian.clone()));

        // Call scatter through the Material enum
        let (color, _) = lambertian.scatter(&ray, &hit_record);

        // Verify we got the right color back
        assert_eq!(color, albedo);
    }
}
