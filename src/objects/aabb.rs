use nalgebra_glm::{TVec3, e, TVec2, vec3, vec2};
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct AABB {
    pub min: TVec3<f32>,
    pub max: TVec3<f32>,
    pub center : TVec3<f32>,
    pub size: TVec3<f32>
}

impl AABB {

    pub fn new(min: TVec3<f32>,max: TVec3<f32>) -> AABB {
        let width_x = (max.x - min.x) * 0.5;
        let width_y = (max.y - min.y) * 0.5;
        let width_z = (max.z - min.z) * 0.5;

        let center = vec3(min.x + width_x,min.y + width_y, min.z + width_z);

        let scale = vec3(width_x,width_y,width_z);

        return AABB { min, max, center, size: scale };
    }

    pub fn collision_test_point(&self, c: &TVec3<f32>) -> bool {

       return c.x >= self.min.x && c.x <= self.max.x && c.y >= self.min.y && c.y <= self.max.y && c.z >= self.min.z && c.z <= self.max.z;
    }

    pub fn collision_test(&self, c: &AABB) -> bool {

        let epsilon = 0.0001f32;
        //let epsilon = 0.0;

        if c.max.x - epsilon <= self.min.x || c.min.x + epsilon >= self.max.x {
            return false;
        }
        if c.max.y - epsilon <= self.min.y || c.min.y + epsilon >= self.max.y {
            return false;
        }
        if c.max.z - epsilon <= self.min.z || c.min.z + epsilon >= self.max.z {
            return false;
        }

        return true;
    }

    pub fn collision_test_no_y(&self, c: &AABB) -> bool {

        let epsilon = 0.0001f32;
        //let epsilon = 0.0;

        if c.max.x - epsilon <= self.min.x || c.min.x + epsilon >= self.max.x {
            return false;
        }
        if c.max.z - epsilon <= self.min.z || c.min.z + epsilon >= self.max.z {
            return false;
        }

        return true;
    }

    pub fn collision_test_no_z(&self, c: &AABB) -> bool {

        let epsilon = 0.0001f32;
        //let epsilon = 0.0;

        if c.max.x - epsilon <= self.min.x || c.min.x + epsilon >= self.max.x {
            return false;
        }
        if c.max.y - epsilon <= self.min.y || c.min.y + epsilon >= self.max.y {
            return false;
        }

        return true;
    }

    pub fn update_center(&mut self) {
        let width_x = (self.max.x - self.min.x) * 0.5;
        let width_y = (self.max.y - self.min.y) * 0.5;
        let width_z = (self.max.z - self.min.z) * 0.5;

        let center = vec3(self.min.x + width_x,self.min.y + width_y, self.min.z + width_z);

        self.center = center;
    }
}

#[derive(Clone)]
pub struct RoundBB {
    pub center: TVec3<f32>,
    pub radius: f32
}

impl RoundBB {

    pub fn new(pos: TVec3<f32>, radius:f32) -> RoundBB {
        return RoundBB { center: pos,radius};
    }

    pub fn collision_test(&self, c: &RoundBB) -> bool {

        let x = (c.center.x - self.center.x).abs();
        let y = (c.center.y - self.center.y).abs();
        let z = (c.center.z - self.center.z).abs();

        let distance = (x.powf(2.0) + y.powf(2.0) + z.powf(2.0)).sqrt();

        return distance <= self.radius + c.radius;
    }

    pub fn collision_test_point(&self, c: &TVec3<f32>) -> bool {

        let x = (c.x - self.center.x).abs();
        let y = (c.y - self.center.y).abs();
        let z = (c.z - self.center.z).abs();

        let distance = (x.powf(2.0) + y.powf(2.0) + z.powf(2.0)).sqrt();

        return distance <= self.radius;
    }

    pub fn collision_test_aabb(&self, c: &AABB) -> bool {

        let circle_distance: TVec2<f32> = vec2((self.center.x - c.center.x).abs(),(self.center.y - c.center.y).abs());

        if circle_distance.x > (self.radius + c.size.x) {
            return false
        }
        if circle_distance.y > (self.radius + c.size.y) {
            return false
        }

        if circle_distance.x <= c.size.x { return true; }
        if circle_distance.y <= c.size.y { return true; }

        let circle_distance_sq : f32 = (circle_distance.x - c.size.x).powf(2.0) +
            (circle_distance.y - c.size.y).powf(2.0);

        return circle_distance_sq <= self.radius.powf(2.0);
    }
}

