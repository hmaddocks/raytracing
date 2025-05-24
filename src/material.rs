use crate::color::Color;
use crate::hittable::HitRecord;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::utilities::random_double;
use crate::vec3::Vec3;
use std::fmt;

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    Test(TestMaterial),
}

impl fmt::Debug for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Material::Lambertian(_) => write!(f, "Material::Lambertian"),
            Material::Metal(_) => write!(f, "Material::Metal"),
            Material::Dielectric(_) => write!(f, "Material::Dielectric"),
            Material::Test(_) => write!(f, "Material::Test"),
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Material::Lambertian(_), Material::Lambertian(_))
                | (Material::Metal(_), Material::Metal(_))
                | (Material::Dielectric(_), Material::Dielectric(_))
                | (Material::Test(_), Material::Test(_))
        )
    }
}

impl Clone for Material {
    fn clone(&self) -> Self {
        match self {
            Material::Lambertian(l) => Material::Lambertian(l.clone()),
            Material::Metal(m) => Material::Metal(m.clone()),
            Material::Dielectric(d) => Material::Dielectric(d.clone()),
            Material::Test(t) => Material::Test(t.clone()),
        }
    }
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Color, Ray) {
        match self {
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Metal(m) => m.scatter(ray, hit_record),
            Material::Dielectric(d) => d.scatter(ray, hit_record),
            Material::Test(t) => t.scatter(ray, hit_record),
        }
    }
}

pub struct Lambertian {
    texture: Box<dyn Texture>,
}

impl Clone for Lambertian {
    fn clone(&self) -> Self {
        Self {
            texture: Box::new(SolidColor::new(self.texture.value(
                0.0,
                0.0,
                &Point3::new(0.0, 0.0, 0.0),
            ))),
        }
    }
}

impl Lambertian {
    pub fn new(texture: Box<dyn Texture>) -> Material {
        Material::Lambertian(Lambertian { texture })
    }

    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Color, Ray) {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let time = ray.time();
        let scatter = Ray::new(hit_record.position, scatter_direction, time);
        let attenuation = self
            .texture
            .value(hit_record.u, hit_record.v, &hit_record.position);
        (attenuation, scatter)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Material {
        let fuzz = fuzz.clamp(0.0, 1.0);
        Material::Metal(Metal { albedo, fuzz })
    }

    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Color, Ray) {
        let mut reflected = ray.direction().reflect(&hit_record.normal);
        reflected = reflected.unit() + (Vec3::random_unit() * self.fuzz);
        let time = ray.time();
        let scatter = Ray::new(hit_record.position, reflected, time);
        (self.albedo, scatter)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Material {
        Material::Dielectric(Dielectric { refraction_index })
    }

    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Color, Ray) {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction().unit();
        let cos_theta = (-unit_direction.dot(&hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            unit_direction.reflect(&hit_record.normal)
        } else {
            unit_direction.refract(&hit_record.normal, ri)
        };

        let time = ray.time();
        (attenuation, Ray::new(hit_record.position, direction, time))
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

// A simple material for testing purposes
#[derive(Clone, Debug, PartialEq)]
pub struct TestMaterial;

impl TestMaterial {
    pub fn new() -> Material {
        Material::Test(TestMaterial)
    }

    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Color, Ray) {
        // Simple implementation that just returns white and a ray in the normal direction
        let scatter_direction = hit_record.normal;
        let time = ray.time();
        let scatter = Ray::new(hit_record.position, scatter_direction, time);
        (Color::new(1.0, 1.0, 1.0), scatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point3::Point3;

    // Helper function to create a HitRecord for testing
    fn create_hit_record(position: Point3, normal: Vec3, material: Option<Material>) -> HitRecord {
        let hit_record = HitRecord {
            position,
            normal,
            t: 1.0,
            front_face: true,
            material,
            ..Default::default()
        };
        hit_record
    }

    #[test]
    fn test_lambertian_creation() {
        let texture = SolidColor::new(Color::new(0.5, 0.5, 0.5));
        let material = Lambertian::new(Box::new(texture.clone()));

        match material {
            Material::Lambertian(l) => {
                // Check that the material was created successfully
                assert!(
                    l.texture.value(0.0, 0.0, &Point3::new(0.0, 0.0, 0.0))
                        == texture.value(0.0, 0.0, &Point3::new(0.0, 0.0, 0.0))
                );
            }
            _ => panic!("Expected Lambertian material"),
        }
    }

    #[test]
    fn test_lambertian_scatter() {
        let texture = SolidColor::new(Color::new(0.5, 0.5, 0.5));
        let material = Lambertian::new(Box::new(texture.clone()));

        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 0.0);
        let hit_point = Point3::new(0.0, 0.0, 1.0);
        let normal = Vec3::new(0.0, 0.0, -1.0); // Surface normal pointing back

        let hit_record = create_hit_record(hit_point, normal, Some(material.clone()));

        let (scattered_color, scattered_ray) = match material {
            Material::Lambertian(l) => l.scatter(&ray, &hit_record),
            _ => panic!("Expected Lambertian material"),
        };

        // Check that the scattered color is the texture color
        assert_eq!(
            scattered_color,
            texture.value(0.0, 0.0, &Point3::new(0.0, 0.0, 0.0))
        );

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
        let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), ray_dir, 0.0);

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
        let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), ray_dir, 0.0);

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
            direction_length > 0.0 && direction_length < 3.0,
            "Expected direction length between 0.0 and 3.0, got: {}",
            direction_length
        );

        // Also verify that the direction is not zero
        assert!(
            !scattered_ray.direction().near_zero(),
            "Scattered ray direction should not be near zero"
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

        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 0.0);
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

        let texture = SolidColor::new(Color::new(0.5, 0.5, 0.5));
        let lambertian = Lambertian::new(Box::new(texture.clone()));

        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 0.0);
        let hit_point = Point3::new(0.0, 0.0, 1.0);
        let normal = Vec3::new(0.0, 0.0, -1.0);

        let hit_record = create_hit_record(hit_point, normal, Some(lambertian.clone()));

        // Call scatter through the Material enum
        let (color, _) = lambertian.scatter(&ray, &hit_record);

        // Verify we got the right color back
        assert_eq!(color, texture.value(0.0, 0.0, &Point3::new(0.0, 0.0, 0.0)));
    }
}
