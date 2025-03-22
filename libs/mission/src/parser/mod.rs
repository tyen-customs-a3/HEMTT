use std::collections::HashMap;

use hemtt_config::{parse as parse_config_file, ConfigReport};
use hemtt_preprocessor::Processor;
use hemtt_sqm::{SqmFile, parse_sqm as sqm_parser};
use hemtt_sqf::parser::{database::Database, self};
use hemtt_workspace::{WorkspacePath, reporting::{Processed, Output}};
use tracing::debug;

use crate::error::Error;
use crate::files::{FileType, MissionFile};

/// Scan all files in the mission directory
///
/// # Errors
/// Returns an error if the directory cannot be scanned
pub fn scan_files(path: &WorkspacePath) -> Result<HashMap<FileType, Vec<WorkspacePath>>, Error> {
    let mut files = HashMap::new();
    
    // Walk through all files in the mission directory
    for file in path.walk_dir()? {
        if let Some(file_type) = FileType::from_path(&file) {
            files.entry(file_type).or_insert_with(Vec::new).push(file);
        }
    }
    
    Ok(files)
}

/// Parse a file based on its type
///
/// # Errors
/// Returns an error if the file cannot be parsed
pub fn parse_file(file_type: FileType, path: WorkspacePath) -> Result<MissionFile, Error> {
    match file_type {
        FileType::Sqm => parse_sqm(&path),
        FileType::Config(_) => parse_config(&path),
        FileType::Script => parse_script(&path),
        FileType::StringTable => parse_stringtable(&path),
        FileType::Other(_) => {
            let content = if file_type.is_text() {
                path.read_to_string()?
            } else {
                String::new()
            };
            Ok(MissionFile::new(path, content))
        }
    }
}

/// Parse a mission.sqm file
///
/// # Errors
/// Returns an error if the file cannot be parsed
fn parse_sqm(path: &WorkspacePath) -> Result<MissionFile, Error> {
    debug!("Parsing mission.sqm: {}", path);
    
    let content = path.read_to_string()?;
    let sqm_file = sqm_parser(&content).map_err(|e| Error::SqmParse(format!("{:?}", e)))?;
    
    Ok(MissionFile::with_sqm(path.clone(), content, sqm_file))
}

/// Parse a config file
///
/// # Errors
/// Returns an error if the file cannot be parsed
pub fn parse_config(path: &WorkspacePath) -> Result<MissionFile, Error> {
    debug!("Parsing config file: {}", path);
    
    // Read the raw content
    let content = path.read_to_string()?;
    
    // Process the config file through the preprocessor first
    let processed = Processor::run(path)
        .map_err(|e| Error::ConfigParse(format!("Preprocessor error: {:?}", e)))?;
    
    // Parse the preprocessed content with the config parser
    let config_report = parse_config_file(None, &processed)
        .map_err(|e| Error::ConfigParse(format!("Config parse error: {:?}", e)))?;
    
    Ok(MissionFile::with_config(path.clone(), content, config_report))
}

/// Parse a SQF script file
///
/// # Errors
/// Returns an error if the file cannot be read
fn parse_script(path: &WorkspacePath) -> Result<MissionFile, Error> {
    debug!("Parsing script file: {}", path);
    
    // Read the raw content
    let content = path.read_to_string()?;
    
    // Process the script through the preprocessor first
    let processed = Processor::run(path)
        .map_err(|e| Error::ScriptParse(format!("Preprocessor error: {:?}", e)))?;
    
    // Parse the preprocessed content with the SQF parser
    let database = Database::a3(false);
    let statements = match parser::run(&database, &processed) {
        Ok(statements) => statements,
        Err(parser::ParserError::LexingError(e)) => return Err(Error::ScriptLexError(e)),
        Err(parser::ParserError::ParsingError(e)) => return Err(Error::ScriptParseError(e)),
    };
    
    Ok(MissionFile::with_script(path.clone(), content, statements))
}

/// Parse a stringtable file
///
/// # Errors
/// Returns an error if the file cannot be read
fn parse_stringtable(path: &WorkspacePath) -> Result<MissionFile, Error> {
    debug!("Reading stringtable file: {}", path);
    let content = path.read_to_string()?;
    Ok(MissionFile::with_stringtable(path.clone(), content))
} 