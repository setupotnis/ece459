fn main() {
    let v = vec![String::from("q"), String::from("r")];
    let (returned_v, len) = calculate_length(v);
    println!("The length of the {:?} is {}", returned_v, len);
}

fn calculate_length(v: Vec<String>) -> (Vec<String>, usize) {
    let len = v.len();
    return (v, len);
}
