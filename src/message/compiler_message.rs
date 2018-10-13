use serde::{Deserialize, Deserializer, Serialize, Serializer};

// See struct Diagnostic (src/libsyntax/json.rs) in rust.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerMessage {
    pub message: String,
    pub code: Option<DiagnosticCode>,
    pub level: ErrorLevel,
    pub spans: Vec<DiagnosticSpan>,
    pub children: Vec<CompilerMessage>,
    pub rendered: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSpan {
    pub file_name: String,
    pub byte_start: u32,
    pub byte_end: u32,
    pub line_start: usize,
    pub line_end: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub is_primary: bool,
    pub text: Vec<DiagnosticSpanLine>,
    pub label: Option<String>,
    pub suggested_replacement: Option<String>,
    pub suggestion_applicability: Option<Applicability>,
    pub expansion: Option<Box<DiagnosticSpanMacroExpansion>>,
}

#[derive(Debug, Clone)]
pub enum Applicability {
    MachineApplicable,
    HasPlaceholders,
    MaybeIncorrect,
    Unspecified,
    Other(String),
}

impl Serialize for Applicability {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use self::Applicability::*;
        let name = match *self {
            MachineApplicable => "MachineApplicable",
            HasPlaceholders => "HasPlaceholders",
            MaybeIncorrect => "MaybeIncorrect",
            Unspecified => "Unspecified",
            Other(ref name) => name,
        };
        name.serialize(s)
    }
}

impl<'de> Deserialize<'de> for Applicability {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use self::Applicability::*;
        let name = String::deserialize(d)?;
        match name.as_str() {
            "MachineApplicable" => return Ok(MachineApplicable),
            "HasPlaceholders" => return Ok(HasPlaceholders),
            "MaybeIncorrect" => return Ok(MaybeIncorrect),
            "Unspecified" => return Ok(Unspecified),
            _ => {}
        };
        Ok(Other(name))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSpanLine {
    pub text: String,
    pub highlight_start: usize,
    pub highlight_end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSpanMacroExpansion {
    pub span: DiagnosticSpan,
    pub macro_decl_name: String,
    pub def_site_span: Option<DiagnosticSpan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticCode {
    pub code: String,
    pub explanation: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ErrorLevel {
    InternalCompilerError,
    Error,
    Warning,
    Note,
    Help,
    Other(String),
}

impl Serialize for ErrorLevel {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use self::ErrorLevel::*;
        let name = match *self {
            InternalCompilerError => "error: internal compiler error",
            Error => "error",
            Warning => "warning",
            Note => "note",
            Help => "help",
            Other(ref name) => name,
        };
        name.serialize(s)
    }
}

impl<'de> Deserialize<'de> for ErrorLevel {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use self::ErrorLevel::*;
        let name = String::deserialize(d)?;
        match name.as_str() {
            "error: internal compiler error" => return Ok(InternalCompilerError),
            "error" => return Ok(Error),
            "warning" => return Ok(Warning),
            "note" => return Ok(Note),
            "help" => return Ok(Help),
            _ => {}
        };
        Ok(Other(name))
    }
}
