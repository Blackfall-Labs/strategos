/// Archive format detection and abstraction layer
pub mod detection;
pub mod traits;

pub mod engram;
pub mod cartridge;
pub mod dataspool;
pub mod datacard;

// Re-export main types
pub use detection::{ArchiveFormat, detect_format};
pub use traits::{Archive, MutableArchive, QueryableArchive, ArchiveInfo, FileEntry, SearchResult};
pub use engram::EngramArchive;
pub use cartridge::CartridgeArchive;
pub use dataspool::DataSpoolArchive;
pub use datacard::DataCardArchive;
