    enum Shape{
    Circle(f64),
    Rectangle(f64,f64),
}

fn main(){
   let a = Shape::Circle(2.0);
   let b = Shape::Rectangle(5.0, 7.0);

  let a1 = calculate_area(&a);
  let a2 = calculate_area(&b);

  println!("Area of a is {}",a1);
  println!("Area of b is {}",a2);
}

fn calculate_area(shape : &Shape) -> f64{
    match shape{
        Shape::Circle(r) => 3.14*r*r,
        Shape::Rectangle(a,b) => a*b,
    }
}