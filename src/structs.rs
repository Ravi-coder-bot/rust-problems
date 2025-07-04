struct User {
  name : String , 
  last_name : String , 
  age : u8 , 
}

fn main(){
  let user1 = User{
    name : String::from("Ravi"), 
    last_name : String::from("Beniwal"), 
    age : 20 ,
  };

  println!("user1 have these details : {} {} {}", user1.name,user1.last_name,user1.age)
}