pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

struct Texture {
    pub color: Color,
}

impl Texture {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            color: Color::new(r, g, b),
        }
    }
}

impl Texture for Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.color
    }
}

struct CheckerTexture {
    pub odd: Box<dyn Texture>,
    pub even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>) -> Self {
        Self { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
