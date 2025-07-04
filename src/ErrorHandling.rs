use std::fs::read_to_string;

fn main() {
    let ans = read_from_file(String::from("input.txt"));

    match ans {
        Ok(contents) => println!("File Contents:\n{}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}

fn read_from_file(path: String) -> Result<String, String> {
    let res = read_to_string(path);
    match res {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}
