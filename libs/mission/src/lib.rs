//! HEMTT Mission Processing Library
//! 
//! This library provides tools for processing Arma 3 mission folders
//! including parsing mission.sqm files, SQF scripts, description.ext,
//! and other mission-specific files.

mod error;
pub mod files;
mod mission;
pub mod parser;
mod diagnostics;

pub use error::Error;
pub use files::{FileType, ConfigFileType, OtherFileType, MissionFile};
pub use mission::Mission;
pub use diagnostics::{scan_and_print_diagnostics, has_parsing_errors, get_error_counts};

// Re-export commonly used types
pub use hemtt_sqm::SqmFile;
pub use hemtt_config::ConfigReport;
pub use hemtt_workspace::WorkspacePath;