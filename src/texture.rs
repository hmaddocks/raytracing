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
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        let sines =
            (self.scale * p.x()).sin() * (self.scale * p.y()).sin() * (self.scale * p.z()).sin();
        if sines > 0.0 {
            self.odd.value(_u, _v, p)
        } else {
            self.even.value(_u, _v, p)
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

        let texture = CheckerTexture::new(std::f64::consts::PI, odd, even); // Use scale PI for clear sign
        // Points where sines > 0 (odd)
        let p1 = Point3::new(0.5, 0.5, 0.5);
        let sines1 = (std::f64::consts::PI * p1.x()).sin()
            * (std::f64::consts::PI * p1.y()).sin()
            * (std::f64::consts::PI * p1.z()).sin();
        println!("sines1: {}", sines1);
        assert!(sines1 > 0.0);
        assert_eq!(texture.value(0.0, 0.0, &p1), odd_color);
        // Points where sines < 0 (even)
        let p2 = Point3::new(1.5, 0.5, 0.5);
        let sines2 = (std::f64::consts::PI * p2.x()).sin()
            * (std::f64::consts::PI * p2.y()).sin()
            * (std::f64::consts::PI * p2.z()).sin();
        println!("sines2: {}", sines2);
        assert!(sines2 < 0.0);
        assert_eq!(texture.value(0.0, 0.0, &p2), even_color);
    }

    #[test]
    fn test_checker_texture_scale() {
        let odd_color = Color::new(1.0, 1.0, 1.0);
        let even_color = Color::new(0.0, 0.0, 0.0);
        let odd = Box::new(TextureEnum::SolidColor(SolidColor::new(odd_color)));
        let even = Box::new(TextureEnum::SolidColor(SolidColor::new(even_color)));

        let texture = CheckerTexture::new(std::f64::consts::PI, odd, even);
        // Points where sines > 0 (odd)
        let p1 = Point3::new(0.25, 0.25, 0.25);
        let sines1 = (std::f64::consts::PI * p1.x()).sin()
            * (std::f64::consts::PI * p1.y()).sin()
            * (std::f64::consts::PI * p1.z()).sin();
        println!("sines1: {}", sines1);
        assert!(sines1 > 0.0);
        assert_eq!(texture.value(0.0, 0.0, &p1), odd_color);
        // Points where sines < 0 (even)
        let p2 = Point3::new(1.25, 0.25, 0.25);
        let sines2 = (std::f64::consts::PI * p2.x()).sin()
            * (std::f64::consts::PI * p2.y()).sin()
            * (std::f64::consts::PI * p2.z()).sin();
        println!("sines2: {}", sines2);
        assert!(sines2 < 0.0);
        assert_eq!(texture.value(0.0, 0.0, &p2), even_color);
    }

    #[test]
    fn test_checker_texture_pattern() {
        let odd_color = Color::new(1.0, 1.0, 1.0); // White
        let even_color = Color::new(0.0, 0.0, 0.0); // Black
        let odd = Box::new(TextureEnum::SolidColor(SolidColor::new(odd_color)));
        let even = Box::new(TextureEnum::SolidColor(SolidColor::new(even_color)));

        let texture = CheckerTexture::new(std::f64::consts::PI, odd, even);
        // Points where sines > 0 (odd)
        let p1 = Point3::new(0.75, 0.75, 0.75);
        let sines1 = (std::f64::consts::PI * p1.x()).sin()
            * (std::f64::consts::PI * p1.y()).sin()
            * (std::f64::consts::PI * p1.z()).sin();
        println!("sines1: {}", sines1);
        assert!(sines1 > 0.0);
        assert_eq!(texture.value(0.0, 0.0, &p1), odd_color);
        // Points where sines < 0 (even)
        let p2 = Point3::new(1.75, 0.75, 0.75);
        let sines2 = (std::f64::consts::PI * p2.x()).sin()
            * (std::f64::consts::PI * p2.y()).sin()
            * (std::f64::consts::PI * p2.z()).sin();
        println!("sines2: {}", sines2);
        assert!(sines2 < 0.0);
        assert_eq!(texture.value(0.0, 0.0, &p2), even_color);
    }
}
