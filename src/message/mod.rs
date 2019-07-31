use serde::{Deserialize, Serialize};

pub mod compiler_message;
mod package_id;
mod target;

pub use self::compiler_message::CompilerMessage;
pub use self::package_id::PackageId;
pub use self::target::{Edition, Target};

// See trait Message (src/cargo/util/machine_message.rs) in cargo.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "reason")]
pub enum Message {
    #[serde(rename = "compiler-message")]
    FromCompiler(FromCompiler),
    #[serde(rename = "compiler-artifact")]
    Artifact(Artifact),
    #[serde(rename = "build-script-executed")]
    BuildScript(BuildScript),
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FromCompiler {
    pub package_id: PackageId,
    pub target: Target,
    pub message: CompilerMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub package_id: PackageId,
    pub target: Target,
    pub profile: ArtifactProfile,
    pub features: Vec<String>,
    pub filenames: Vec<String>,
    pub fresh: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactProfile {
    pub opt_level: String,
    pub debuginfo: Option<u32>,
    pub debug_assertions: bool,
    pub overflow_checks: bool,
    pub test: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildScript {
    pub package_id: PackageId,
    pub linked_libs: Vec<String>,
    pub linked_paths: Vec<String>,
    pub cfgs: Vec<String>,
    pub env: Vec<(String, String)>,
}
