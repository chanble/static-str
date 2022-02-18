use static_str::to_str;

fn main() {
    let option_value = Some(123);
    let v = option_value.map_or("", |v| to_str(v.to_string()));
    assert_eq!(v, "123");
}