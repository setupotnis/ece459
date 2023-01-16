fn main() {
    let s: String = String::from("hello");
    let s1: String = modify(s);
    println!("{}", s1);
}

fn modify(mut s: String) -> String {
    s.push_str(" world");
    return s;
}
