//!
//! convert a string into a static str
//! and string to &str
//! 
//! fix cannot return reference to temporary value
//! ```rust
//! fn string_to_static_str_works() {
//!     let option_value = Some(123);
//!     // cannot return reference to temporary value
//!     // let v = option_value.map_or("", |v| v.to_string().as_str());
//!     let v = option_value.map_or("", |v| to_str(v.to_string()));
//!     assert_eq!(v, "123");
//! }
//! ```

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn to_str(s: String) -> &'static str {
    string_to_static_str(s)
}

mod tests {

    use super::*;

    #[test]
    fn string_to_static_str_works() {
        let option_value = Some(123);
        let v = option_value.map_or("", |v| to_str(v.to_string()));
        assert_eq!(v, "123");
    }
}