enum Shape{
    Rectangle(f64, f64),
    Circle(f64)
}

fn main(){
    let rect = Shape::Rectangle(20.1, 3.8);
    let circle = Shape::Circle(10.2);

    println!("Area of Rectangle is: {}", find_area(rect));
    println!("Area of a Circle is: {}", find_area(circle));
}

fn find_area(shape:Shape) -> f64{
    match shape{
        Shape::Rectangle(a, b) => a*b,
        Shape::Circle(a) => 3.14*a*a
    }
}
