use std::collections::HashMap;

use hemtt_config::{parse as parse_config_file, ConfigReport};
use hemtt_preprocessor::{process, Options as PreprocessorOptions};
use hemtt_sqm::{SqmFile, parse_sqm as sqm_parser};
use hemtt_workspace::{reporting::Processed, WorkspacePath};
use tracing::debug;

use crate::error::Error;
use crate::files::{FileType, ConfigFileType, OtherFileType};

/// Scan all files in the mission directory
///
/// # Errors
/// Returns an error if the directory cannot be scanned
pub fn scan_files(path: &WorkspacePath) -> Result<HashMap<FileType, Vec<WorkspacePath>>, Error> {
    let mut files = HashMap::new();
    
    // Walk through all files in the mission directory
    for file in path.walk_dir()? {
        let file_type = FileType::from_path(&file);
        files.entry(file_type).or_insert_with(Vec::new).push(file);
    }
    
    Ok(files)
}

/// Parse a mission.sqm file
///
/// # Errors
/// Returns an error if the file cannot be parsed
pub fn parse_sqm(path: &WorkspacePath) -> Result<(String, SqmFile), Error> {
    debug!("Parsing mission.sqm: {}", path);
    
    let content = path.read_to_string()?;
    let sqm_file = sqm_parser(&content).map_err(|e| Error::SqmParse(format!("{:?}", e)))?;
    
    Ok((content, sqm_file))
}

/// Parse a config file (hpp, ext, etc.)
///
/// # Errors
/// Returns an error if the file cannot be parsed
pub fn parse_config(path: &WorkspacePath) -> Result<(String, ConfigReport), Error> {
    debug!("Parsing config file: {}", path);
    
    // Read the raw content
    let content = path.read_to_string()?;
    
    // Process the config file through the preprocessor first
    let options = PreprocessorOptions::default();
    let processed = process(&content, &options, path.as_str())
        .map_err(|e| Error::ConfigParse(format!("Preprocessor error: {}", e)))?;
    
    // Create a Processed object required by the config parser
    let processed = Processed::from_string(processed.to_string());
    
    // Parse the preprocessed content with the config parser
    let config_report = parse_config_file(None, &processed)
        .map_err(|e| Error::ConfigParse(format!("Config parse error: {:?}", e)))?;
    
    Ok((content, config_report))
}

/// Parse a SQF script file - currently just reads the file
///
/// # Errors
/// Returns an error if the file cannot be read
pub fn parse_script(path: &WorkspacePath) -> Result<String, Error> {
    debug!("Reading script file: {}", path);
    let content = path.read_to_string()?;
    Ok(content)
}

/// Parse a stringtable file
///
/// # Errors
/// Returns an error if the file cannot be read
pub fn parse_stringtable(path: &WorkspacePath) -> Result<String, Error> {
    debug!("Reading stringtable file: {}", path);
    let content = path.read_to_string()?;
    Ok(content)
} 