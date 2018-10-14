extern crate semver;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate xml;

pub mod checkstyle;
pub mod message;

use std::io::{self, Cursor};

use xml::writer::EventWriter;

use checkstyle::CheckstyleDoc;

fn main() {
    let stdin = Cursor::new(include_bytes!("sample.txt").to_vec());
    let checkstyle = CheckstyleDoc::from_reader(stdin).expect("I/O error when reading input");

    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut xml = EventWriter::new(stdout);
    checkstyle.write_xml(&mut xml).expect("Error writing XML");
}
