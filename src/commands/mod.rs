//! CLI command implementations

// Shared commands (work across all formats)
pub mod shared;

// Format-specific commands
pub mod cartridge;
pub mod dataspool;
pub mod datacard;

// Legacy Engram-specific commands
pub mod extract;
pub mod info;
pub mod keygen;
pub mod list;
pub mod pack;
pub mod query;
pub mod search;
pub mod sign;
pub mod verify;
