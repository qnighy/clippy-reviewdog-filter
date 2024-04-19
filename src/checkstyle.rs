use std::collections::HashMap;
use std::io::{self, BufRead, Write};

use xml::writer::{Error as EmitterError, EventWriter, XmlEvent};

use crate::message::compiler_message::ErrorLevel;
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct Options {
    pub include_rendered: bool,
    pub redirect_to_stderr: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CheckstyleDoc {
    pub files: HashMap<String, CheckstyleFile>,
}

impl CheckstyleDoc {
    pub fn from_reader<R: BufRead>(r: R, opts: &Options) -> io::Result<Self> {
        let mut checkstyle = Self::default();

        for line in r.lines() {
            let line = line?;
            if !line.starts_with("{") {
                if opts.redirect_to_stderr {
                    eprintln!("{}", line.trim_end_matches("\n"));
                }
                continue;
            }
            let msg: Message = serde_json::from_str(&line)?;
            checkstyle.append_message(&msg, opts);
        }
        Ok(checkstyle)
    }

    pub fn append_message(&mut self, msg: &Message, opts: &Options) {
        let msg = if let Message::FromCompiler(msg) = msg {
            msg
        } else {
            return;
        };
        let (file, column, line) = if let Some(ref span) = msg.message.spans.get(0) {
            (
                span.file_name.clone(),
                span.column_start as u32,
                span.line_start as u32,
            )
        } else {
            return;
        };
        let severity = match msg.message.level {
            ErrorLevel::InternalCompilerError => "error",
            ErrorLevel::Error => "error",
            ErrorLevel::Warning => "warning",
            ErrorLevel::Note => "info",
            ErrorLevel::Help => "info",
            ErrorLevel::Other(_) => "error",
        };
        let file_entry = self.files.entry(file).or_default();
        let mut message = msg.message.message.clone();
        if opts.include_rendered {
            if let Some(ref rendered) = msg.message.rendered {
                message.push_str("\n\n```\n");
                message.push_str(rendered);
                message.push_str("\n```\n");
            }
        }
        file_entry.errors.push(CheckstyleError {
            column: column,
            line: line,
            message: message,
            severity: severity.to_owned(),
            source: msg.message.code.as_ref().map(|code| code.code.clone()),
        });
    }
    pub fn write_xml<W: Write>(&self, xml: &mut EventWriter<W>) -> Result<(), EmitterError> {
        xml.write(XmlEvent::start_element("checkstyle"))?;
        for (filename, file) in &self.files {
            file.write_xml(xml, filename)?;
        }
        xml.write(XmlEvent::end_element())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct CheckstyleFile {
    pub errors: Vec<CheckstyleError>,
}

impl CheckstyleFile {
    pub fn write_xml<W: Write>(
        &self,
        xml: &mut EventWriter<W>,
        name: &str,
    ) -> Result<(), EmitterError> {
        xml.write(XmlEvent::start_element("file").attr("name", name))?;
        for error in &self.errors {
            error.write_xml(xml)?;
        }
        xml.write(XmlEvent::end_element())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CheckstyleError {
    pub column: u32,
    pub line: u32,
    pub message: String,
    pub severity: String,
    pub source: Option<String>,
}

impl CheckstyleError {
    pub fn write_xml<W: Write>(&self, xml: &mut EventWriter<W>) -> Result<(), EmitterError> {
        let column = self.column.to_string();
        let line = self.line.to_string();
        let elem = XmlEvent::start_element("error")
            .attr("column", &column)
            .attr("line", &line)
            .attr("message", &self.message)
            .attr("severity", &self.severity);
        let elem = if let Some(ref source) = self.source {
            elem.attr("source", source)
        } else {
            elem
        };
        xml.write(elem)?;
        xml.write(XmlEvent::end_element())?;
        Ok(())
    }
}
