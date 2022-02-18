rust string to &'static str
=========================================

how-to-convert-a-string-into-a-static-str

[copy form here(how-to-convert-a-string-into-a-static-str)](https://stackoverflow.com/a/30527289)

> cannot return reference to temporary value


## usage:

```rust 
use static_str::to_str;

fn main() {
    let option_value = Some(123);
    let v = option_value.map_or("", |v| to_str(v.to_string()));
    assert_eq!(v, "123");
}

```