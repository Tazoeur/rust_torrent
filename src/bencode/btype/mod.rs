pub mod byte;
pub mod int;
pub mod list;
pub mod map;

pub trait BencodeElement {
    fn encode(&self) -> String;

    fn parse(content: &str) -> Self;
}
