//

use lazy_static::*;
use regex::Regex;

pub struct BString {
    value: String,
}

impl BString {
    pub fn new(value: String) -> BString {
        BString { value }
    }

    pub fn encode(&self) -> String {
        BString::encode_from_value(&self.value)
    }

    fn encode_from_value(value: &str) -> String {
        let byte_count = value.as_bytes().len();
        format!("{}:{}", byte_count, value)
    }

    pub fn parse(content: &str) -> Result<BString, &str> {
        // use lazy_static to prevent the regex to be compiled at each parsing
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<size>\d+):(?P<value>.*)$").unwrap();
        }

        if let Some(capture) = RE.captures(content) {
            let size = capture
                .name("size")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let text = capture.name("value").unwrap().as_str();

            if size != text.as_bytes().len() {
                Err("Wrong size association")
            } else {
                Ok(BString {
                    value: text.to_string(),
                })
            }
        } else {
            Err("The content given is not a bencoded byte string")
        }
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

#[cfg(test)]
mod tests {
    use super::BString;

    #[test]
    fn parse_spam() {
        let result = BString::parse("4:spam");
        assert_eq!("spam".to_string(), result.unwrap().get_value());
    }

    #[test]
    fn parse_empty_string() {
        let result = BString::parse("0:");
        assert_eq!("".to_string(), result.unwrap().get_value());
    }

    #[test]
    fn encode_spam() {
        let result = BString::encode_from_value("spam");
        assert_eq!("4:spam", &result);
    }

    #[test]
    fn encode_empty_string() {
        let result = BString::encode_from_value("");
        assert_eq!("0:", &result);
    }
}
