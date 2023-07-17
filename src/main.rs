#[derive(Debug, Clone)]
enum Shapes {
    Rectangle(u32, u32),
    Circle {
        radius: f32
    }
}

impl Shapes {
    fn square(size: u32) -> Self {
        Self::Rectangle(size, size)
    }

    fn area(&self) -> f32 {
        match self {
            Shapes::Rectangle(width, height) => (width * height) as f32,
            Shapes::Circle { radius } => 2f32 * radius * 3.14
        }
    }

    fn sides(&self) -> Option<u32> {
        if let Shapes::Rectangle(_, _) = self {
            Some(4)
        } else {
            None
        }
    }
}

fn main() {
    let rect = Shapes::Rectangle(30, 50);
    let circle = Shapes::Circle { radius: 13.0 };
    let square = Shapes::square(50);

    dbg!(rect.area(), rect.sides(), circle.area(), circle.sides(), square.area(), square.sides());

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s2 + &s1; // note s1 has been moved here and can no longer be used
    println!("{s3}");
    println!("{s3}");
}
