use env_logger;

pub mod checkstyle;
pub mod message;

use std::io;

use clap::{App, Arg};
use xml::writer::EventWriter;

use crate::checkstyle::CheckstyleDoc;

fn main() {
    env_logger::init();

    let args = App::new("clippy-reviewdog-filter")
        .version("0.1.1")
        .author("Masaki Hara <ackie.h.gmai@gmail.com>")
        .about("Converts cargo check / cargo clippy output into checkstyle-like XML.")
        .arg(
            Arg::with_name("include-rendered")
                .long("include-rendered")
                .help("include rendered messages"),
        )
        .get_matches();

    let options = checkstyle::Options {
        include_rendered: args.is_present("include-rendered"),
        redirect_to_stderr: true,
    };

    let stdin = io::stdin();
    let stdin = stdin.lock();
    let checkstyle =
        CheckstyleDoc::from_reader(stdin, &options).expect("I/O error when reading input");

    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut xml = EventWriter::new(stdout);
    checkstyle.write_xml(&mut xml).expect("Error writing XML");
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    #[test]
    fn test_from_reader() -> io::Result<()> {
        let options = checkstyle::Options {
            include_rendered: false,
            redirect_to_stderr: true,
        };
        let reader = Cursor::new(include_bytes!("sample.txt").to_vec());
        let checkstyle = CheckstyleDoc::from_reader(reader, &options)?;
        assert_eq!(checkstyle.files.len(), 1);
        let file = &checkstyle.files["src/main.rs"];
        assert_eq!(file.errors.len(), 2);

        let error0 = &file.errors[0];
        assert_eq!(error0.column, 9);
        assert_eq!(error0.line, 3);
        assert_eq!(error0.message, "unused variable: `x`");
        assert_eq!(error0.severity, "warning");
        assert_eq!(error0.source, Some("unused_variables".to_owned()));

        let error1 = &file.errors[1];
        assert_eq!(error1.column, 13);
        assert_eq!(error1.line, 3);
        assert_eq!(
            error1.message,
            "casting u32 to f64 may become silently lossy if types change"
        );
        assert_eq!(error1.severity, "warning");
        assert_eq!(error1.source, Some("cast_lossless".to_owned()));

        Ok(())
    }
}
