use static_str;

fn main() {
    let s = "hello";
    let ss = format!("{}", s);

    let s2 = static_str::string_to_static_str(ss);

    println!("s: {}, s2: {}", s, s2);
}