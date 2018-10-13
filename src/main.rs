extern crate semver;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate quick_xml;

pub mod message;

use message::Message;

fn main() {
    let msg = include_str!("sample.json");
    let msg: Message = serde_json::from_str(msg).unwrap();
    println!("msg = {:?}", msg);
}
