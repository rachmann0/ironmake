use std::ffi::OsStr;
use std::time::SystemTime;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum ArtifactType {
    // ! triple slash (///) doc comments for hover tooltips
    /// Source file (.c)
    Header,
    /// Source file (.c)
    Source,
    /// Compiled object file (.o, .obj)
    Object,
    /// Final executable binary
    Binary,
    /// Shared library (.so, .dll, .dylib)
    SharedLib,
    /// Static library (.a, .lib)
    StaticLib,

    /// invalid ext
    Other,

    // /// Packaged library, module, or archive (JAR, wheel, zip)
    // Package,
    // /// Generated documentation
    // Documentation,
    // /// Intermediate file like preprocessed code
    // Intermediate,
    // /// Any other custom type
    // Other(String),
}

impl ArtifactType {
    pub fn classify(path: &Path) -> ArtifactType{
    let ext = match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => ext,
        None => return ArtifactType::Binary, // no ext
    };

    match ext {
        "h" | "hpp" => ArtifactType::Header,
        "c" | "cpp" => ArtifactType::Source,
        "o" => ArtifactType::Object,
        "exe" => ArtifactType::Binary,
        "dll" | "so" | "dylib" => ArtifactType::SharedLib,
        "a" | "lib" => ArtifactType::StaticLib,
        _ => ArtifactType::Other // invalid ext
    }
}
}

#[derive(Debug, Clone)]
pub struct Artifact {
    /// Path to the artifact file
    pub path: PathBuf,
    /// Type of artifact (e.g., "binary", "object", "library", "package")
    pub artifact_type: ArtifactType,
    /// Timestamp of creation
    pub created_at: SystemTime,
    /// Optional metadata (like compiler flags, source files, hash)
    pub metadata: Option<String>,

    pub dependancies: Vec<Artifact>,
    // pub target: Option<Artifact>,
    pub is_built:bool
}

impl Artifact {
    /// Create a new artifact
    // pub fn new(path: PathBuf, artifact_type: ArtifactType, metadata: Option<String>) -> Self {
    pub fn new(
        path: PathBuf, metadata: Option<String>,
        dependancies: Vec<Artifact>,
        // dependancies2: Vec<usize>,
        is_built:bool
    ) -> Self {

        Self {
            artifact_type: match ArtifactType::classify(&path) {
                ArtifactType::Other=>panic!("Unsupported file extension: {:?}", path),
                default => default
            },
            path,
            created_at: SystemTime::now(),
            metadata,
            dependancies,
            // target,
            is_built,
        }
    }


    // /// Check if the artifact exists on disk
    // pub fn exists(&self) -> bool {
    //     self.path.exists()
    // }
}