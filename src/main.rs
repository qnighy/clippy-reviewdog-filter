extern crate semver;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate xml;

pub mod message;

use std::collections::HashMap;
use std::io::{self, BufRead, Cursor, Write};

use xml::writer::{Error as EmitterError, EventWriter, XmlEvent};

use message::compiler_message::ErrorLevel;
use message::Message;

fn main() {
    let stdin = Cursor::new(include_bytes!("sample.txt").to_vec());
    let mut files: HashMap<String, Vec<_>> = HashMap::new();

    for line in stdin.lines() {
        let line = line.expect("I/O error when reading input");
        if !line.starts_with("{") {
            continue;
        }
        let msg: Message = serde_json::from_str(&line).unwrap();
        let msg = if let Message::FromCompiler(msg) = msg {
            msg
        } else {
            continue;
        };
        let (file, column, line) = if let Some(ref span) = msg.message.spans.get(0) {
            (
                span.file_name.clone(),
                span.column_start as u32,
                span.line_start as u32,
            )
        } else {
            ("<unknown>".to_owned(), 1, 1)
        };
        let severity = match msg.message.level {
            ErrorLevel::InternalCompilerError => "error",
            ErrorLevel::Error => "error",
            ErrorLevel::Warning => "warning",
            ErrorLevel::Note => "info",
            ErrorLevel::Help => "info",
            ErrorLevel::Other(_) => "error",
        };
        files.entry(file).or_default().push(CheckstyleError {
            column: column,
            line: line,
            message: msg.message.message.clone(),
            severity: severity.to_owned(),
            source: msg.message.code.as_ref().map(|code| code.code.clone()),
        });
    }

    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut xml = EventWriter::new(stdout);
    xml.write(XmlEvent::start_element("checkstyle")).unwrap();
    for (file, errors) in &files {
        xml.write(XmlEvent::start_element("file").attr("name", file))
            .unwrap();
        for error in errors {
            let column = error.column.to_string();
            let line = error.line.to_string();
            let elem = XmlEvent::start_element("error")
                .attr("column", &column)
                .attr("line", &line)
                .attr("message", &error.message)
                .attr("severity", &error.severity);
            let elem = if let Some(ref source) = error.source {
                elem.attr("source", source)
            } else {
                elem
            };
            xml.write(elem).unwrap();
            xml.write(XmlEvent::end_element()).unwrap();
        }
        xml.write(XmlEvent::end_element()).unwrap();
    }
    xml.write(XmlEvent::end_element()).unwrap();
}

#[derive(Debug, Clone)]
pub struct CheckstyleError {
    pub column: u32,
    pub line: u32,
    pub message: String,
    pub severity: String,
    pub source: Option<String>,
}
