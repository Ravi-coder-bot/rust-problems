struct Rect{
   width : u32 , 
  length : u32 , 
}

impl Rect{
  fn area(&self) -> u32{
    self.width * self.length
  }
}

fn main(){
  let rect1 = Rect{
    width : 30 ,
    length : 50 ,
  };
  println!("The area of the rectangle is {} square pixels." , rect1.area());
}