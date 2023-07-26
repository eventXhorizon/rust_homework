pub enum Shape {
    Circle(Circle),
    Rect(Rect),
    Triangle(Triangle)
}

pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub struct Rect {
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub struct Triangle {
    pub width: f64,
    pub height: f64,
}

impl Triangle {
    pub fn area(&self) -> f64 {
        (self.width * self.height) / 2.0
    }
}