use crate::color::Color;
use crate::point3::Point3;

#[derive(Clone)]
pub enum TextureEnum {
    SolidColor(SolidColor),
    CheckerTexture(CheckerTexture),
}

impl Texture for TextureEnum {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        match self {
            TextureEnum::SolidColor(t) => t.value(u, v, p),
            TextureEnum::CheckerTexture(t) => t.value(u, v, p),
        }
    }
}

/// A trait representing a texture that can be applied to surfaces.
/// Textures are used to determine the color of a point on a surface
/// based on its UV coordinates and position.
pub trait Texture: Send + Sync {
    /// Returns the color at the given UV coordinates and point in 3D space.
    ///
    /// # Arguments
    /// * `u` - The U coordinate in texture space
    /// * `v` - The V coordinate in texture space
    /// * `p` - The point in 3D space
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color;
}

/// A texture that returns a constant color regardless of position or UV coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct SolidColor {
    /// The constant color to return
    pub color: Color,
}

impl SolidColor {
    /// Creates a new solid color texture with the given color.
    ///
    /// # Arguments
    /// * `color` - The constant color to use for this texture
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl From<Color> for SolidColor {
    fn from(color: Color) -> Self {
        Self::new(color)
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    pub scale: f64,
    pub odd: Box<TextureEnum>,
    pub even: Box<TextureEnum>,
}

impl CheckerTexture {
    /// Creates a new checker texture with the given scale and odd/even textures.
    ///
    /// # Arguments
    /// * `scale` - The scale of the checker pattern. Must be positive.
    /// * `odd` - The texture to use for odd squares
    /// * `even` - The texture to use for even squares
    ///
    /// # Panics
    /// Panics if `scale` is not positive.
    pub fn new(scale: f64, odd: Box<TextureEnum>, even: Box<TextureEnum>) -> Self {
        assert!(scale > 0.0, "Scale must be positive");
        Self { scale, odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_integer = (p.x() * self.scale).floor() as i32;
        let y_integer = (p.y() * self.scale).floor() as i32;
        let z_integer = (p.z() * self.scale).floor() as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_color_texture() {
        let color = Color::new(0.5, 0.3, 0.1);
        let texture = SolidColor::new(color);
        let point = Point3::new(1.0, 2.0, 3.0);

        // Test that the texture always returns the same color regardless of coordinates
        assert_eq!(texture.value(0.0, 0.0, &point), color);
        assert_eq!(texture.value(0.5, 0.5, &point), color);
        assert_eq!(texture.value(1.0, 1.0, &point), color);
    }

    #[test]
    fn test_checker_texture() {
        let odd_color = Color::new(1.0, 1.0, 1.0); // White
        let even_color = Color::new(0.0, 0.0, 0.0); // Black
        let odd = Box::new(TextureEnum::SolidColor(SolidColor::new(odd_color)));
        let even = Box::new(TextureEnum::SolidColor(SolidColor::new(even_color)));

        let texture = CheckerTexture::new(1.0, odd, even);

        // Test points that should be in even squares
        let even_point1 = Point3::new(0.0, 0.0, 0.0);
        let even_point2 = Point3::new(2.0, 2.0, 0.0);
        assert_eq!(texture.value(0.0, 0.0, &even_point1), even_color);
        assert_eq!(texture.value(0.0, 0.0, &even_point2), even_color);

        // Test points that should be in odd squares
        let odd_point1 = Point3::new(1.0, 0.0, 0.0);
        let odd_point2 = Point3::new(0.0, 1.0, 0.0);
        assert_eq!(texture.value(0.0, 0.0, &odd_point1), odd_color);
        assert_eq!(texture.value(0.0, 0.0, &odd_point2), odd_color);
    }

    #[test]
    fn test_checker_texture_scale() {
        let odd_color = Color::new(1.0, 1.0, 1.0);
        let even_color = Color::new(0.0, 0.0, 0.0);
        let odd = Box::new(TextureEnum::SolidColor(SolidColor::new(odd_color)));
        let even = Box::new(TextureEnum::SolidColor(SolidColor::new(even_color)));

        // Create a checker texture with scale 2.0
        let texture = CheckerTexture::new(2.0, odd, even);

        // Test points with the new scale
        let point1 = Point3::new(1.0, 1.0, 0.0); // Should be even
        let point2 = Point3::new(2.5, 1.0, 0.0); // Should be odd
        assert_eq!(texture.value(0.0, 0.0, &point1), even_color);
        assert_eq!(texture.value(0.0, 0.0, &point2), odd_color);
    }

    #[test]
    fn test_checker_texture_clone() {
        let odd_color = Color::new(1.0, 1.0, 1.0);
        let even_color = Color::new(0.0, 0.0, 0.0);
        let odd = Box::new(TextureEnum::SolidColor(SolidColor::new(odd_color)));
        let even = Box::new(TextureEnum::SolidColor(SolidColor::new(even_color)));

        let texture = CheckerTexture::new(1.0, odd, even);
        let cloned_texture = texture.clone();

        // Test that the cloned texture produces the same results
        let point = Point3::new(1.0, 1.0, 0.0);
        assert_eq!(
            texture.value(0.0, 0.0, &point),
            cloned_texture.value(0.0, 0.0, &point)
        );
    }
}
