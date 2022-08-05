#[derive(Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub u: f32,
    pub v: f32
}

impl Vertex {
    pub fn new(x:f32,y:f32,z:f32,u:f32,v:f32) -> Vertex {
        return Vertex {
            x,
            y,
            z,
            u,
            v
        };
    }
}

#[derive(Clone)]
pub struct NormalVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub u: f32,
    pub v: f32,
    pub n_x: f32,
    pub n_y: f32,
    pub n_z: f32
}

impl NormalVertex {
    pub fn new(x:f32,y:f32,z:f32,u:f32,v:f32,n_x: f32,n_y: f32,n_z: f32) -> NormalVertex {
        return NormalVertex {
            x,
            y,
            z,
            u,
            v,
            n_x,
            n_y,
            n_z
        };
    }
}
#[derive(Clone)]
pub struct OnlyCoordsVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl OnlyCoordsVertex {
    pub fn new(x:f32,y:f32,z:f32) -> OnlyCoordsVertex {
        return OnlyCoordsVertex {
            x,
            y,
            z
        };
    }
}