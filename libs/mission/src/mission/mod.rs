use std::collections::HashMap;
use std::sync::Arc;

use hemtt_workspace::{WorkspacePath, reporting::{Codes, Code}};
use tracing::{info, debug};
use rayon::prelude::*;

use crate::error::Error;
use crate::files::{FileType, ConfigFileType, MissionFile};
use crate::parser;

mod metadata;
use metadata::MetadataExtractor;

/// Main entry point for processing Arma 3 missions
#[derive(Debug)]
pub struct Mission {
    /// Path to the mission folder
    path: WorkspacePath,
    /// All files in the mission
    files: HashMap<FileType, Vec<MissionFile>>,
    /// Errors and warnings encountered during processing
    codes: Codes,
}

impl Mission {
    /// Create a new mission from a WorkspacePath
    /// 
    /// # Errors
    /// Returns an error if the mission cannot be processed
    pub fn new(path: WorkspacePath) -> Result<Self, Error> {
        let mut mission = Self {
            path: path.clone(),
            files: HashMap::new(),
            codes: Vec::new(),
        };

        mission.process()?;
        Ok(mission)
    }

    /// Get the workspace path for this mission
    #[must_use]
    pub fn workspace(&self) -> &WorkspacePath {
        &self.path
    }

    /// Process the mission folder
    fn process(&mut self) -> Result<(), Error> {
        info!("Processing mission at {}", self.path);
        
        // Initialize file collections
        self.initialize_file_collections();
        
        // Scan and process files
        let scanned_files = parser::scan_files(&self.path)?;
        
        // Process each file type in parallel
        for (file_type, paths) in scanned_files {
            debug!("Processing {} files", file_type.to_string());
            
            // Process files of the same type in parallel
            let results: Vec<Result<MissionFile, Error>> = paths.par_iter()
                .map(|path| parser::parse_file(file_type, path.clone()))
                .collect();
            
            // Handle results
            for result in results {
                match result {
                    Ok(mission_file) => {
                        self.files.entry(file_type).or_default().push(mission_file);
                    },
                    Err(e) => {
                        debug!("Failed to parse file: {}", e);
                        
                        // Get the path from the error if possible, or use a placeholder
                        let path_str = match &e {
                            Error::Workspace(we) => we.to_string(),
                            Error::SqmParse(p) => p.clone(),
                            Error::ConfigParse(p) => p.clone(),
                            Error::ScriptParse(p) => p.clone(),
                            Error::ScriptLexError(_) => "SQF lexing error".to_string(),
                            Error::ScriptParseError(_) => "SQF parsing error".to_string(),
                            Error::StringtableParse(p) => p.clone(),
                            Error::MissingFiles(p) => p.clone(),
                            Error::Generic(p) => p.clone(),
                            Error::Analysis(_) => "Analysis error".to_string(),
                        };
                        
                        self.codes.push(Arc::new(MissionError::new(
                            file_type.error_ident(),
                            format!("Failed to parse {}: {}", path_str, e)
                        )));
                        
                        // If we have parsing errors from SQF, add those directly
                        if let Error::ScriptLexError(codes) | Error::ScriptParseError(codes) = e {
                            for code in codes {
                                self.codes.push(code);
                            }
                        }
                    }
                }
            }
        }
        
        info!("Mission processing complete");
        Ok(())
    }

    /// Initialize collections for each file type
    fn initialize_file_collections(&mut self) {
        for file_type in FileType::all() {
            self.files.insert(file_type, Vec::new());
        }
    }
    
    /// Get mission.sqm file
    #[must_use]
    pub fn sqm(&self) -> Option<&MissionFile> {
        self.files.get(&FileType::Sqm)?.first()
    }
    
    /// Get description.ext file
    #[must_use]
    pub fn description_ext(&self) -> Option<&MissionFile> {
        self.files.get(&FileType::Config(ConfigFileType::Ext))?.first()
    }
    
    /// Get all script files
    #[must_use]
    pub fn script_files(&self) -> &[MissionFile] {
        self.files.get(&FileType::Script).map_or(&[], |v| v.as_slice())
    }
    
    /// Get all stringtable files
    #[must_use]
    pub fn stringtable_files(&self) -> &[MissionFile] {
        self.files.get(&FileType::StringTable).map_or(&[], |v| v.as_slice())
    }
    
    /// Get all config files of a specific type
    #[must_use]
    pub fn config_files(&self, config_type: ConfigFileType) -> &[MissionFile] {
        self.files.get(&FileType::Config(config_type)).map_or(&[], |v| v.as_slice())
    }
    
    /// Get all files of a specific type
    #[must_use]
    pub fn files_of_type(&self, file_type: FileType) -> &[MissionFile] {
        self.files.get(&file_type).map_or(&[], |v| v.as_slice())
    }
    
    /// Get all files in the mission
    #[must_use]
    pub fn all_files(&self) -> Vec<&MissionFile> {
        self.files.values().flat_map(|files| files.iter()).collect()
    }
    
    /// Get errors and warnings encountered during processing
    #[must_use]
    pub fn codes(&self) -> &Codes {
        &self.codes
    }
    
    /// Get mission name from mission.sqm
    #[must_use]
    pub fn name(&self) -> Option<String> {
        self.sqm().and_then(|sqm| sqm.name())
    }
    
    /// Get mission author from mission.sqm
    #[must_use]
    pub fn author(&self) -> Option<String> {
        self.sqm().and_then(|sqm| sqm.author())
    }
}

/// Mission error struct for reporting issues
#[derive(Debug)]
struct MissionError {
    ident: &'static str,
    message: String,
}

impl MissionError {
    fn new(ident: &'static str, message: impl Into<String>) -> Self {
        Self {
            ident,
            message: message.into(),
        }
    }
}

impl Code for MissionError {
    fn ident(&self) -> &'static str {
        self.ident
    }

    fn message(&self) -> String {
        self.message.clone()
    }

    fn severity(&self) -> hemtt_workspace::reporting::Severity {
        hemtt_workspace::reporting::Severity::Error
    }
} 