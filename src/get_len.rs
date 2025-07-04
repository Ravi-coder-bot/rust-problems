fn main(){
    let name = String::from("Ravi");
    let len = get_len(name);
    println!("{}",len);
}

fn get_len(s : String) -> usize{
    return s.chars().count(); 

}