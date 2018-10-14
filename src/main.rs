extern crate semver;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate xml;

pub mod checkstyle;
pub mod message;

use std::io::{self, BufRead, Cursor};

use xml::writer::EventWriter;

use checkstyle::CheckstyleDoc;
use message::Message;

fn main() {
    let stdin = Cursor::new(include_bytes!("sample.txt").to_vec());
    let mut checkstyle = CheckstyleDoc::default();

    for line in stdin.lines() {
        let line = line.expect("I/O error when reading input");
        if !line.starts_with("{") {
            continue;
        }
        let msg: Message = serde_json::from_str(&line).unwrap();
        checkstyle.append_message(&msg);
    }

    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut xml = EventWriter::new(stdout);
    checkstyle.write_xml(&mut xml).expect("Error writing XML");
}
