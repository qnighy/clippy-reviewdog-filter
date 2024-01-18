use env_logger;

pub mod checkstyle;
pub mod message;

use std::io;

use clap::{Command, Arg, ArgAction};
use xml::writer::EventWriter;

use crate::checkstyle::CheckstyleDoc;

fn main() {
    env_logger::init();

    let args = Command::new("clippy-reviewdog-filter")
        .version("0.1.1")
        .author("Masaki Hara <ackie.h.gmai@gmail.com>")
        .about("Converts cargo check / cargo clippy output into checkstyle-like XML.")
        .arg(
            Arg::new("include-rendered")
                .long("include-rendered")
                .help("include rendered messages")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let options = checkstyle::Options {
        include_rendered: args.get_flag("include-rendered"),
        redirect_to_stderr: true,
    };

    let stdin = io::stdin();
    let stdin = stdin.lock();
    let checkstyle =
        CheckstyleDoc::from_reader(stdin, &options).expect("I/O error when reading input");

    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut xml = EventWriter::new(stdout);
    match checkstyle.write_xml(&mut xml) {
        Ok(_) => {}
        Err(xml::writer::Error::Io(e)) if e.kind() == io::ErrorKind::BrokenPipe => {
            // ignore broken pipe
            std::process::exit(141);
        }
        Err(e) => {
            eprintln!("Error writing XML: {}", e);
            std::process::exit(1);
        }
    }
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
