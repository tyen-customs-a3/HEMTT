use std::path::Path;
use hemtt_workspace::WorkspacePath;
use hemtt_config::ConfigReport;
use hemtt_sqm::SqmFile;
use hemtt_sqf::Statements;

mod types;
pub use types::*;

/// Represents a file in the mission
#[derive(Debug, Clone)]
pub struct MissionFile {
    /// Path to the file
    pub path: WorkspacePath,
    /// File content
    pub content: String,
    /// Parsed data (if applicable)
    parsed_data: Option<ParsedData>,
}

/// Different types of parsed data
#[derive(Debug, Clone)]
pub enum ParsedData {
    /// Parsed mission.sqm file
    Sqm(SqmFile),
    /// Parsed config file
    Config(ConfigReport),
    /// Parsed SQF script
    Script(Statements),
    /// Parsed stringtable
    StringTable,
}

impl MissionFile {
    /// Create a new MissionFile without parsed data
    #[must_use]
    pub fn new(path: WorkspacePath, content: String) -> Self {
        Self {
            path,
            content,
            parsed_data: None,
        }
    }

    /// Create a new MissionFile with SQM data
    #[must_use]
    pub fn with_sqm(path: WorkspacePath, content: String, sqm: SqmFile) -> Self {
        Self {
            path,
            content,
            parsed_data: Some(ParsedData::Sqm(sqm)),
        }
    }

    /// Create a new MissionFile with config data
    #[must_use]
    pub fn with_config(path: WorkspacePath, content: String, config: ConfigReport) -> Self {
        Self {
            path,
            content,
            parsed_data: Some(ParsedData::Config(config)),
        }
    }

    /// Create a new MissionFile with script data
    #[must_use]
    pub fn with_script(path: WorkspacePath, content: String, statements: Statements) -> Self {
        Self {
            path,
            content,
            parsed_data: Some(ParsedData::Script(statements)),
        }
    }

    /// Create a new MissionFile with stringtable data
    #[must_use]
    pub fn with_stringtable(path: WorkspacePath, content: String) -> Self {
        Self {
            path,
            content,
            parsed_data: Some(ParsedData::StringTable),
        }
    }

    /// Get SQM data if available
    #[must_use]
    pub fn sqm_data(&self) -> Option<&SqmFile> {
        if let Some(ParsedData::Sqm(sqm)) = &self.parsed_data {
            Some(sqm)
        } else {
            None
        }
    }

    /// Get config data if available
    #[must_use]
    pub fn config_data(&self) -> Option<&ConfigReport> {
        if let Some(ParsedData::Config(config)) = &self.parsed_data {
            Some(config)
        } else {
            None
        }
    }

    /// Get script data if available
    #[must_use]
    pub fn script_data(&self) -> Option<&Statements> {
        if let Some(ParsedData::Script(script)) = &self.parsed_data {
            Some(script)
        } else {
            None
        }
    }
} 