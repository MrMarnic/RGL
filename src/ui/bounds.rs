use nalgebra_glm::{Mat4, TMat4, vec3};
use crate::objects::aabb::AABB;

pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub mat4: Mat4,
    pub aabb: AABB
}

impl Bounds {
    pub fn new(x:f32,y:f32,width:f32,height:f32) -> Self{
        let mut b = Bounds {
            x,
            y,
            width,
            height,
            mat4: nalgebra_glm::identity(),
            aabb: AABB::new(vec3(0.0,0.0,0.0),vec3(0.0,0.0,0.0))
        };
        b.aabb = b.to_aabb();
        b.mat4 = b.create_matrix();

        return b
    }

    pub fn set(&mut self,x:f32,y:f32,width:f32,height:f32) {
        self.x = (x as i32) as f32;
        self.y = (y as i32) as f32;
        self.width = (width as i32) as f32;
        self.height = (height as i32) as f32;
        self.update();
    }

    pub fn to_aabb(&self) -> AABB {
        return AABB::new(vec3(self.x - self.width,self.y - self.height, 0.0),vec3(self.x + self.width, self.y + self.height,0.0));
    }

    pub fn update(&mut self) {
        self.mat4 = self.create_matrix();
        self.aabb = self.to_aabb();
    }

    pub fn left(&self) -> f32 {
        return self.x - self.width;
    }

    pub fn right(&self) -> f32 {
        return self.x + self.width;
    }

    pub fn top(&self) -> f32 {
        return self.y + self.height;
    }

    pub fn bottom(&self) -> f32 {
        return self.y - self.height;
    }

    fn create_matrix(&self) -> TMat4<f32> {
        let mut matrix : TMat4<f32> = nalgebra_glm::identity() as TMat4<f32>;
        matrix = nalgebra_glm::translate(&mut matrix,&vec3(self.x,self.y,0.0));
        matrix = nalgebra_glm::scale(&matrix,&nalgebra_glm::vec3(self.width,self.height,0.0));
        return matrix;
    }
}