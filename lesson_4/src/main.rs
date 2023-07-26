mod trait_case;
mod enum_case;
mod task2;

use crate::enum_case::*;
use crate::trait_case::*;

/*
    主要区别在于:
        1. 枚举方式可以直接存放值，trait object需要放Box，或者要使用用引用 &dyn
        2. 枚举方式需要匹配处理，trait object可以直接调用
        3. trait object有额外的动态分发开销（但开销应该不会差距太大）。
        总结：需要根据实际场景选择合适的方式。
*/

fn main() {

    // trait
    let shapes = vec![
        Box::new(CircleObj{radius: 1.0}) as Box<dyn ShapeTrait>,
        Box::new(RectObj{width: 2.0, height: 3.0}) as Box<dyn ShapeTrait>,
        Box::new(TriangleObj{width: 2.0, height: 3.0}) as Box<dyn ShapeTrait>,

    ];

    for s in &shapes {
        println!("Area: {}", s.area());
    }

    // enum
    let shapes = vec![
        Shape::Circle(Circle { radius: 1.0 }),
        Shape::Rect(Rect { width: 2.0, height: 3.0 }),
        Shape::Triangle(Triangle { width: 2.0, height: 3.0 }),
    ];

    for s in &shapes {
        match s {
            Shape::Circle(c) => println!("Circle area: {}", c.area()),
            Shape::Rect(r) => println!("Rect area: {}", r.area()),
            Shape::Triangle(t) => println!("Rect area: {}", t.area()),
        }
    }
}