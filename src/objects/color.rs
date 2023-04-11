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

    pub const BLACK : Self = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE : Self = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED : Self = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const LIME : Self = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE : Self = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const YELLOW : Self = Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const CYAN : Self = Color {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const MAGENTA : Self = Color {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const SILVER : Self = Color {
        r: 0.75,
        g: 0.75,
        b: 0.75,
        a: 1.0,
    };
    pub const GRAY : Self = Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };
    pub const MAROON : Self = Color {
        r: 0.5,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN : Self = Color {
        r: 0.0,
        g: 0.5,
        b: 0.0,
        a: 1.0,
    };
    pub const PURPLE : Self = Color {
        r: 0.5,
        g: 0.0,
        b: 0.5,
        a: 1.0,
    };
    pub const TEAL : Self = Color {
        r: 0.0,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };
    pub const NAVY : Self = Color {
        r: 0.0,
        g: 0.0,
        b: 0.5,
        a: 1.0,
    };
}