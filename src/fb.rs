fn main(){
    println!("{}", fb(5));           
}


fn fb(n : u32) -> u32{
    let mut first = 0 ;
    let mut second = 1 ;
    if n== 0{
        return first  ;
    }
    else if n == 1{
        return  second;   
    }
    else{
      for _ in 2..=n{
           let temp = second ;
           second = first + second ;
           first = temp ; 
        }
    }

    return second ; 
    
}