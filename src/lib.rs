
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