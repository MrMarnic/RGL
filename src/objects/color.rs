#[derive(Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color {
    pub fn new_with_a(r:i32,g:i32,b:i32,a:f32) -> Color {
        return Color {
            r: r as f32/255.0,
            g: g as f32/255.0,
            b: b as f32/255.0,
            a
        }
    }

    pub fn new(r:i32,g:i32,b:i32) -> Color {

        return Color {
            r: r as f32/255.0,
            g: g as f32/255.0,
            b: b as f32/255.0,
            a: 1.0
        }
    }

    pub fn copy(&mut self,from:&Color) {
        self.r = from.r;
        self.g = from.g;
        self.b = from.b;
        self.a = from.a;
    }

    pub fn a(mut self,a:f32) -> Self {
        self.a = a;
        self
    }
}