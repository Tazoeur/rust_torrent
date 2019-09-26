use lazy_static::*;
use regex::Regex;

#[derive(Debug)]
pub struct Int {
    value: i64,
}

impl Int {
    pub fn encode(&self) -> String {
        Int::encode_from_value(self.value)
    }

    fn encode_from_value(value: i64) -> String {
        format!("i{}e", value)
    }

    pub fn parse(content: &str) -> Result<Int, &str> {
        // use lazy_static to prevent the regex to be compiled at each parsing
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^i(?P<value>(-?[1-9][0-9]*)|0)e$").unwrap();
        }
        // let RE = regex::Regex::new(r"^i(?<value>(-?[1-9][0-9]*)|0)e$").unwrap();

        if let Some(capture) = RE.captures(content) {
            let value = capture
                .name("value")
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap();
            Ok(Int { value })
        } else {
            // if the regex did not match, it means that the content is not a bencoded number
            Err("The content given is not a bencoded number")
        }
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }

    pub fn set_value(&mut self, value: i64) {
        self.value = value;
    }

    pub fn new(value: i64) -> Int {
        Int { value }
    }
}

#[cfg(test)]
mod tests {
    use super::Int;

    #[test]
    fn parse_ten() {
        let result = Int::parse("i10e");
        assert_eq!(10, result.unwrap().get_value());
    }

    #[test]
    fn parse_negative_forty_two() {
        let result = Int::parse("i-42e");
        assert_eq!(-42, result.unwrap().get_value());
    }

    #[test]
    fn parse_zero() {
        let result = Int::parse("i0e");
        assert_eq!(0, result.unwrap().get_value());
    }

    #[test]
    #[should_panic(expected = "The content given is not a bencoded number")]
    fn parse_negative_zero() {
        let _result = Int::parse("i-0e").unwrap();
    }

    #[test]
    #[should_panic(expected = "The content given is not a bencoded number")]
    fn parse_leading_zero() {
        let _result = Int::parse("i03e").unwrap();
    }

    #[test]
    fn encode_ten() {
        let result = Int::encode_from_value(10);
        assert_eq!(result, "i10e");
    }

    #[test]
    fn encode_negative_forty_two() {
        let result = Int::encode_from_value(-42);
        assert_eq!(result, "i-42e");
    }

    #[test]
    fn encode_zero() {
        let result = Int::encode_from_value(0);
        assert_eq!(result, "i0e");
    }

}
