use std::sync::Arc;

use hemtt_workspace::reporting::Code;
use thiserror::Error;

/// Errors that can occur while processing a mission
#[derive(Error, Debug)]
pub enum Error {
    /// Error with the workspace
    #[error("Workspace error: {0}")]
    Workspace(#[from] hemtt_workspace::Error),

    /// Error parsing mission.sqm
    #[error("Error parsing mission.sqm: {0}")]
    SqmParse(String),

    /// Error parsing description.ext or other config files
    #[error("Error parsing config file: {0}")]
    ConfigParse(String),

    /// Error parsing SQF script
    #[error("Error parsing SQF script: {0}")]
    ScriptParse(String),

    /// Error lexing SQF script
    #[error("Error lexing SQF script")]
    ScriptLexError(Vec<Arc<dyn Code>>),

    /// Error parsing SQF script
    #[error("Error parsing SQF script")]
    ScriptParseError(Vec<Arc<dyn Code>>),

    /// Error parsing stringtable
    #[error("Error parsing stringtable: {0}")]
    StringtableParse(String),

    /// Mission is missing required files
    #[error("Mission is missing required files: {0}")]
    MissingFiles(String),

    /// Generic error
    #[error("{0}")]
    Generic(String),
    
    /// Errors from linting or analysis
    #[error("Analysis errors")]
    Analysis(Vec<Arc<dyn Code>>),
}

// Note: We're not implementing From<hemtt_sqm::ScanError> because ScanError 
// doesn't implement Display. Instead, we format it in the scanner module. 