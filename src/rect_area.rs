struct Rect{
   width : u32 , 
  length : u32 , 
}

impl Rect{
  fn area(&self) -> u32{
    self.width * self.length
  }

  fn perimeter(&self, num : u32) -> u32{
    2 * (self.width + self.length)
  }

  fn debug(){
    return 1 ; 
  }
}

fn main(){
  let rect1 = Rect{
    width : 30 ,
    length : 50 ,
  };
  println!("The area of the rectangle is {} square pixels." , rect1.area());
  println!("The perimeter of the rectangle is {} pixels." , rect1.perimeter(1));
  println!("{}",Rect::debug());
}