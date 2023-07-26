// trait object
pub trait ShapeTrait {
    fn area(&self) -> f64;
}

pub struct CircleObj {
    pub radius: f64,
}

impl ShapeTrait for CircleObj {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub struct RectObj {
    pub width: f64,
    pub height: f64,
}

impl ShapeTrait for RectObj {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub struct TriangleObj {
    pub width: f64,
    pub height: f64,
}

impl ShapeTrait for TriangleObj {
    fn area(&self) -> f64 {
        (self.width * self.height) / 2.0
    }
}