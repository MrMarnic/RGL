use crate::ui::bounds::Bounds;

#[derive(PartialEq)]
pub enum UIConstraint {
    FixedValue(f32),
    RelativePosDistance(f32),
    RelativePercentage(f32),
    Equals,
    None
}

impl UIConstraint {
    pub fn calculate(&self, current: f32, parent: f32) -> f32 {
        match self {
            UIConstraint::FixedValue(v) => {
                *v
            }
            UIConstraint::RelativePosDistance(v) => {
                parent+v
            }
            UIConstraint::RelativePercentage(v) => {
                parent * v
            },UIConstraint::Equals => {
                parent
            },
            UIConstraint::None => {
                current
            }
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            UIConstraint::None => {
                true
            },
            _ => false
        }
    }

    pub fn is_fixed(&self) -> bool {
        match self {
            UIConstraint::FixedValue(..) => {
                true
            },
            _ => false
        }
    }
}

pub struct ConstraintSettings {
    pub x_constraint: UIConstraint,
    pub y_constraint: UIConstraint,
    pub width_constraint: UIConstraint,
    pub height_constraint: UIConstraint,
    pub left_constraint: UIConstraint,
    pub right_constraint: UIConstraint,
    pub top_constraint: UIConstraint,
    pub bottom_constraint: UIConstraint,
    pub left_border: UIConstraint,
    pub right_border: UIConstraint,
    pub top_border: UIConstraint,
    pub bottom_border: UIConstraint
}

impl ConstraintSettings {
    pub fn new(x:UIConstraint,y:UIConstraint,width:UIConstraint,height:UIConstraint, left:UIConstraint, right:UIConstraint, top:UIConstraint, bottom:UIConstraint
               , left_b:UIConstraint, right_b:UIConstraint, top_b:UIConstraint, bottom_b:UIConstraint) -> ConstraintSettings {
        ConstraintSettings {
            x_constraint: x,
            y_constraint: y,
            width_constraint: width,
            height_constraint: height,
            left_constraint: left,
            right_constraint: right,
            top_constraint: top,
            bottom_constraint: bottom,
            left_border: left_b,
            right_border: right_b,
            top_border: top_b,
            bottom_border: bottom_b
        }
    }

    pub fn default() -> ConstraintSettings {
        ConstraintSettings {
            x_constraint: UIConstraint::None,
            y_constraint: UIConstraint::None,
            width_constraint: UIConstraint::None,
            height_constraint: UIConstraint::None,
            left_constraint: UIConstraint::None,
            right_constraint: UIConstraint::None,
            top_constraint: UIConstraint::None,
            bottom_constraint: UIConstraint::None,
            left_border: UIConstraint::None,
            right_border: UIConstraint::None,
            top_border: UIConstraint::None,
            bottom_border: UIConstraint::None
        }
    }
}