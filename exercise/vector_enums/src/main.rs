enum Shape {
    Circle(f64),
    Square(f64),
}

fn main() {
    let shapes  = vec!(Shape::Circle(3.0), Shape::Square(2.0));
    let total_areas : f64  = shapes.iter()
    .map(|shape| match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length * length,
    })
    .sum();
    println!("Total area: {}", total_areas);
}
