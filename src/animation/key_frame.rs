use nalgebra_glm::TVec3;

#[derive(Clone)]
pub struct KeyFrame {
    pub time: f32,
    pub pos:TVec3<f32>,
    pub scale:TVec3<f32>,
    pub rotation:TVec3<f32>
}