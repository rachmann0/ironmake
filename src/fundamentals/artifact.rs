use std::ffi::OsStr;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub enum ArtifactType {
    // ! triple slash (///) doc comments for hover tooltips
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

    // /// Packaged library, module, or archive (JAR, wheel, zip)
    // Package,
    // /// Generated documentation
    // Documentation,
    // /// Intermediate file like preprocessed code
    // Intermediate,
    // /// Any other custom type
    // Other(String),
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
}

impl Artifact {
    /// Create a new artifact
    // pub fn new(path: PathBuf, artifact_type: ArtifactType, metadata: Option<String>) -> Self {
    pub fn new(path: PathBuf, metadata: Option<String>) -> Self {
        let source_exts=
        [OsStr::new("c")];
        let object_exts=
        [OsStr::new("o")];
        let binary_exts=
        [OsStr::new("exe")];
        let shared_lib_exts =
        [OsStr::new("dll"), OsStr::new("so"), OsStr::new("dylib")];
        let static_lib_exts=
        [OsStr::new("a"), OsStr::new("lib")];

        Self {
            artifact_type: match path.extension() {
                Some(ext) if source_exts.contains(&ext) => ArtifactType::Source,
                Some(ext) if object_exts.contains(&ext) => ArtifactType::Object,
                Some(ext) if shared_lib_exts.contains(&ext) => ArtifactType::SharedLib,
                Some(ext) if binary_exts.contains(&ext) => ArtifactType::Binary,
                Some(ext) if static_lib_exts.contains(&ext) => ArtifactType::StaticLib,
                Some(ext) => panic!("Unsupported file extension: {:?}", ext),
                None => panic!("File has no extension: {:?}", path),
            },
            path,
            created_at: SystemTime::now(),
            metadata,
        }
    }

    // /// Check if the artifact exists on disk
    // pub fn exists(&self) -> bool {
    //     self.path.exists()
    // }
}