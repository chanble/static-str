//!
//! convert a string into a static str
//! ```rust
//! 
//! fn main() {
//!     let s = "hello";
//!     let ss = format!("{}", s);
//! 
//!     let s2 = static_str::string_to_static_str(ss);
//! 
//!     println!("s: {}, s2: {}", s, s2);
//! }
//! ```

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

mod tests {
    #[test]
    fn string_to_static_str_works() {
        let s_str = "hello";
        let ss = format!("{}", s_str);
        assert_eq!(ss.as_str(), s_str);
    }
}