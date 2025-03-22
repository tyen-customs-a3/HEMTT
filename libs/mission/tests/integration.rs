use hemtt_mission::{Mission, FileType, ConfigFileType};
use hemtt_workspace::{LayerType, Workspace};
use hemtt_common::config::PDriveOption;
use std::path::PathBuf;

/// Test fixture for verifying mission parsing
struct MissionParseResults {
    mission: Mission,
    sqm_valid: bool,
    desc_ext_valid: bool,
    config_files_valid: bool,
    has_scripts: bool,
    total_files: usize,
    parsing_errors: usize,
}

impl MissionParseResults {
    fn new(mission_path: &str) -> Self {
        let mission = setup_mission(mission_path);
        let total_files = mission.all_files().len();
        let parsing_errors = mission.codes().len();

        let mut results = Self {
            mission,
            sqm_valid: false,
            desc_ext_valid: false,
            config_files_valid: false,
            has_scripts: false,
            total_files,
            parsing_errors,
        };

        results.verify();
        results.print_summary(mission_path);
        results
    }

    fn verify(&mut self) {
        // Test mission.sqm
        if let Some(sqm) = self.mission.sqm() {
            if let Some(sqm_data) = sqm.sqm_data() {
                self.sqm_valid = verify_sqm_contents(sqm_data);
            }
        }

        // Test description.ext
        if let Some(desc) = self.mission.description_ext() {
            if let Some(config_data) = desc.config_data() {
                self.desc_ext_valid = verify_config_contents(config_data);
            }
        }

        // Test config files
        for config_type in [ConfigFileType::Code, ConfigFileType::Other] {
            let configs = self.mission.config_files(config_type);
            println!("\nChecking {:?} config files:", config_type);
            for config in configs {
                println!("\nVerifying config file: {}", config.path.filename());
                if let Some(config_data) = config.config_data() {
                    self.config_files_valid |= verify_config_contents(config_data);
                }
            }
        }

        // Check scripts
        self.has_scripts = !self.mission.script_files().is_empty();
    }

    fn print_summary(&self, mission_path: &str) {
        println!("\nMission Parse Results for {}:", mission_path);
        println!("- SQM valid: {}", self.sqm_valid);
        println!("- Description.ext valid: {}", self.desc_ext_valid);
        println!("- Config files valid: {}", self.config_files_valid);
        println!("- Has scripts: {}", self.has_scripts);
        println!("- Total files: {}", self.total_files);
        println!("- Parsing errors: {}", self.parsing_errors);

        // Print file type summary
        println!("\nFiles by type:");
        for file_type in FileType::all() {
            let files = self.mission.files_of_type(file_type);
            if !files.is_empty() {
                println!("- {}: {} files", file_type.to_string(), files.len());
                
                // For scripts and configs, print the actual files
                match file_type {
                    FileType::Script | FileType::Config(_) => {
                        for file in files {
                            println!("  - {}", file.path.filename());
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn verify_mission_parsing(mission_path: &str) -> MissionParseResults {
    MissionParseResults::new(mission_path)
}

fn setup_mission(mission_path: &str) -> Mission {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join(mission_path);

    let workspace = Workspace::builder()
        .physical(&path, LayerType::Source)
        .finish(None, true, &PDriveOption::Disallow)
        .expect("Failed to create workspace");

    Mission::new(workspace).expect("Failed to create mission")
}

fn verify_sqm_contents(sqm_data: &hemtt_sqm::SqmFile) -> bool {
    // Check if we have any classes at all
    if sqm_data.classes.is_empty() {
        println!("SQM contains no classes!");
        return false;
    }

    // Check for common mission classes
    let has_mission = sqm_data.classes.contains_key("Mission");
    let has_entities = sqm_data.classes.contains_key("Entities");
    let has_markers = sqm_data.classes.contains_key("Markers");
    
    println!("\nSQM Class verification:");
    println!("- Mission class present: {}", has_mission);
    println!("- Entities class present: {}", has_entities);
    println!("- Markers class present: {}", has_markers);

    // Check Mission class contents if it exists
    if let Some(mission_classes) = sqm_data.classes.get("Mission") {
        if !mission_classes.is_empty() {
            let mission = &mission_classes[0];
            println!("\nMission class properties:");
            for (key, value) in &mission.properties {
                println!("- {}: {:?}", key, value);
            }
            return true;
        }
    }

    false
}

fn verify_config_contents(config_data: &hemtt_config::ConfigReport) -> bool {
    // Check if we have any properties at all
    let config = config_data.config();
    if config.0.is_empty() {
        println!("Config file contains no properties!");
        return false;
    }

    println!("\nConfig File verification:");
    println!("- Total properties: {}", config.0.len());
    
    // Print first few properties for inspection
    println!("\nFirst few properties:");
    for (i, prop) in config.0.iter().take(5).enumerate() {
        println!("{}. {:?}", i + 1, prop);
    }

    // Check for any errors or warnings
    let errors = config_data.errors();
    let warnings = config_data.warnings();
    println!("\nDiagnostics:");
    println!("- Errors: {}", errors.len());
    println!("- Warnings: {}", warnings.len());

    // Print any errors if they exist
    if !errors.is_empty() {
        println!("\nErrors found:");
        for error in errors.iter() {
            println!("- {}: {}", error.ident(), error.message());
        }
    }

    // Check for any patches
    let patches = config_data.patches();
    println!("- Required patches: {}", patches.len());
    
    // Check for localized strings
    let localized = config_data.localized();
    println!("- Localized strings: {}", localized.len());

    // Return true if we have properties and no errors
    !config.0.is_empty() && errors.is_empty()
}

#[test]
fn test_adv48_joust_mission() {
    let results = verify_mission_parsing("adv48_Joust.VR");
    
    assert!(results.sqm_valid, "SQM file does not contain expected data");
    assert!(results.desc_ext_valid, "description.ext does not contain expected data");
    assert!(results.config_files_valid, "No valid config files found with expected data");
    assert!(results.has_scripts, "No script files found");
    assert_eq!(results.parsing_errors, 0, "Found parsing errors");
    
    // Test mission metadata
    assert!(results.mission.name().is_some(), "Mission name not found");
    assert!(results.mission.author().is_some(), "Mission author not found");
}

#[test]
fn test_guerilla_raid_mission() {
    let results = verify_mission_parsing("co22_guerilla_raid.Altis");
    
    assert!(results.sqm_valid, "SQM file does not contain expected data");
    assert!(results.desc_ext_valid, "description.ext does not contain expected data");
    assert!(results.config_files_valid, "No valid config files found with expected data");
    assert!(results.has_scripts, "No script files found");
    assert_eq!(results.parsing_errors, 0, "Found parsing errors");
    
    // Test mission metadata
    assert!(results.mission.name().is_some(), "Mission name not found");
    assert!(results.mission.author().is_some(), "Mission author not found");
}

#[test]
fn test_mission_file_counts() {
    let joust = verify_mission_parsing("adv48_Joust.VR");
    let guerilla = verify_mission_parsing("co22_guerilla_raid.Altis");

    // Basic sanity checks
    assert!(joust.total_files > 0, "Joust mission has no files");
    assert!(guerilla.total_files > 0, "Guerilla mission has no files");
    assert_eq!(joust.parsing_errors, 0, "Joust mission has parsing errors");
    assert_eq!(guerilla.parsing_errors, 0, "Guerilla mission has parsing errors");
} 