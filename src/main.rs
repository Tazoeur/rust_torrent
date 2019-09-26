extern crate lazy_static;
extern crate lib;

use lib::bencode;
// use lib::magnet::Magnet;

fn main() {
    let my_int = bencode::btype::int::Int::parse("i32e");
    println!("{:?}", my_int);
}
