use crate::objects::vertex::{Vertex, NormalVertex};

pub struct VertexBufferData {
    pub vertecies: Vec<Vertex>,
    pub indecies: Vec<u32>
}

impl VertexBufferData {
    pub fn new(vertecies: Vec<Vertex>, indecies: Vec<u32>) -> VertexBufferData {
        return VertexBufferData {
            vertecies,
            indecies
        }
    }
}

pub struct NormalVertexBufferData {
    pub vertecies: Vec<NormalVertex>,
    pub indecies: Vec<u32>
}

impl NormalVertexBufferData {
    pub fn new(vertecies: Vec<NormalVertex>, indecies: Vec<u32>) -> NormalVertexBufferData {
        return NormalVertexBufferData {
            vertecies,
            indecies
        }
    }
}