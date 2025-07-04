enum CustomOption {
  Some(i32),
  None,
}

fn main() {
  let s = String::from("hello world");
  let index = find_first_ch(s);
  match index {
      CustomOption::Some(i) => println!("found at index {}", i),
      CustomOption::None => println!("not found"),
  }
}

fn find_first_ch(s: String) -> CustomOption {
  for (index, char) in s.chars().enumerate() {
      if char == 'a' {
          return CustomOption::Some(index as i32);
      }
  }
  CustomOption::None
}
