rust string to &'static str
=========================================

how-to-convert-a-string-into-a-static-str

[copy form here(how-to-convert-a-string-into-a-static-str)](https://stackoverflow.com/a/30527289)

## usage:

```rust 
use static_str;

fn main() {
    let s = "hello";
    let ss = format!("{}", s);

    let s2 = static_str::string_to_static_str(ss);

    println!("s: {}, s2: {}", s, s2);
}

```