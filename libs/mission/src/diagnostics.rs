use std::path::{Path, PathBuf};
use colored::Colorize;
use hemtt_workspace::{WorkspacePath, reporting::{Severity, Code}};
use crate::{Mission, Error};

/// Scan a mission directory and print diagnostics for any parsing errors
pub fn scan_and_print_diagnostics(path: impl AsRef<Path>) -> Result<(), Error> {
    let path_buf = path.as_ref().to_path_buf();
    let workspace_path = WorkspacePath::slim(&path_buf)?;
    let mission = Mission::new(workspace_path)?;
    
    print_mission_summary(&mission);
    print_parsing_diagnostics(&mission);
    
    Ok(())
}

/// Print a summary of the mission files
fn print_mission_summary(mission: &Mission) {
    println!("\n{}", "Mission Summary:".bold());
    
    // Print mission metadata if available
    if let Some(name) = mission.name() {
        println!("Name: {}", name);
    }
    if let Some(author) = mission.author() {
        println!("Author: {}", author);
    }
    
    // Print total file count
    let total_files = mission.all_files().len();
    println!("\nTotal files: {}", total_files);
    
    // Print counts by type
    println!("\n{}:", "Files by type:".bold());
    for file_type in crate::FileType::all() {
        let files = mission.files_of_type(file_type);
        if !files.is_empty() {
            println!("- {}: {} files", file_type.to_string(), files.len());
            
            // For all files, print the actual files with their paths and class/property counts
            let mut paths = files.iter()
                .map(|file| {
                    let mut info = file.path.as_str().trim_start_matches('/').to_string();
                    
                    // Add class and property counts for SQM and Config files
                    if let Some(sqm_data) = file.sqm_data() {
                        let mut total_classes = 0;
                        let mut total_properties = 0;
                        
                        // Count classes and properties recursively
                        fn count_recursive(classes: &std::collections::HashMap<String, Vec<hemtt_sqm::Class>>, total_classes: &mut usize, total_properties: &mut usize) {
                            for class_vec in classes.values() {
                                *total_classes += class_vec.len();
                                for class in class_vec {
                                    *total_properties += class.properties.len();
                                    count_recursive(&class.classes, total_classes, total_properties);
                                }
                            }
                        }
                        
                        count_recursive(&sqm_data.classes, &mut total_classes, &mut total_properties);
                        info.push_str(&format!(" ({} classes, {} properties)", total_classes, total_properties));
                    } else if let Some(config_data) = file.config_data() {
                        let mut total_classes = 0;
                        let mut total_properties = 0;
                        
                        // Count classes and properties recursively for config files
                        fn count_config_recursive(properties: &[hemtt_config::Property], total_classes: &mut usize, total_properties: &mut usize) {
                            for prop in properties {
                                match prop {
                                    hemtt_config::Property::Class(class) => {
                                        *total_classes += 1;
                                        if let hemtt_config::Class::Local { properties, .. } = class {
                                            count_config_recursive(properties, total_classes, total_properties);
                                        }
                                    }
                                    hemtt_config::Property::Entry { .. } => {
                                        *total_properties += 1;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        
                        count_config_recursive(&config_data.config().0, &mut total_classes, &mut total_properties);
                        info.push_str(&format!(" ({} classes, {} properties)", total_classes, total_properties));
                    } else if let Some(script_data) = file.script_data() {
                        // Count statements and expressions in SQF files
                        let total_statements = script_data.content().len();
                        let total_expressions = script_data.content().iter()
                            .flat_map(|stmt| stmt.walk_expressions())
                            .count();
                        
                        // Count different types of statements
                        let mut assignments = 0;
                        let mut expressions = 0;
                        
                        for statement in script_data.content() {
                            match statement {
                                hemtt_sqf::Statement::AssignGlobal(_, _, _) |
                                hemtt_sqf::Statement::AssignLocal(_, _, _) => assignments += 1,
                                hemtt_sqf::Statement::Expression(_, _) => expressions += 1,
                            }
                        }
                        
                        info.push_str(&format!(" ({} statements [{} assignments, {} expressions], {} total expressions)", 
                            total_statements, assignments, expressions, total_expressions));
                    }
                    
                    info
                })
                .collect::<Vec<_>>();
            paths.sort();
            
            for path in paths {
                println!("  - {}", path);
            }
        }
    }
    
    // Print directory structure for scripts and configs
    println!("\n{}:", "Directory Structure:".bold());
    if let Ok(all_paths) = mission.workspace().walk_dir() {
        let mut dirs = all_paths.iter()
            .filter(|p| p.is_dir().unwrap_or(false))
            .map(|p| p.as_str().trim_start_matches('/').to_string())
            .collect::<Vec<_>>();
        dirs.sort();
        
        for dir in dirs {
            if !dir.is_empty() {
                println!("  📁 {}", dir);
            }
        }
    }
}

/// Print diagnostics for any parsing errors
fn print_parsing_diagnostics(mission: &Mission) {
    let codes = mission.codes();
    
    if codes.is_empty() {
        println!("\n{}", "No parsing errors found.".green());
        return;
    }
    
    println!("\n{}", "Parsing Errors:".red().bold());
    
    let mut error_count = 0;
    let mut warning_count = 0;
    
    for code in codes {
        match code.severity() {
            Severity::Error => {
                error_count += 1;
                println!("{}: {}", "ERROR".red().bold(), code.message());
            }
            Severity::Warning => {
                warning_count += 1;
                println!("{}: {}", "WARNING".yellow().bold(), code.message());
            }
            _ => println!("{}: {}", code.ident(), code.message()),
        }
        
        if let Some(help) = code.help() {
            println!("  Help: {}", help);
        }
        if let Some(note) = code.note() {
            println!("  Note: {}", note);
        }
    }
    
    println!("\nFound {} errors and {} warnings", 
        error_count.to_string().red().bold(),
        warning_count.to_string().yellow().bold()
    );
}

/// Check if a mission has any parsing errors
pub fn has_parsing_errors(mission: &Mission) -> bool {
    mission.codes().iter().any(|c| c.severity() == Severity::Error)
}

/// Get a count of parsing errors and warnings
pub fn get_error_counts(mission: &Mission) -> (usize, usize) {
    let mut errors = 0;
    let mut warnings = 0;
    
    for code in mission.codes() {
        match code.severity() {
            Severity::Error => errors += 1,
            Severity::Warning => warnings += 1,
            _ => {}
        }
    }
    
    (errors, warnings)
} 