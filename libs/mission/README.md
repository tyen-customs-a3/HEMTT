# HEMTT Mission Crate

The HEMTT Mission crate provides functionality for processing Arma 3 mission folders. It can parse mission files, including mission.sqm, description.ext, and various script files, and provides an API for querying mission contents.

## Features

- Direct integration with other HEMTT crates:
  - Uses `hemtt-sqm` for parsing mission.sqm files
  - Uses `hemtt-config` for parsing config files
  - Uses `hemtt-sqf` for script analysis
  - Uses `hemtt-stringtable` for localization support
- Detection and categorization of mission file types
- Access to raw parsed data structures from other crates
- Integration with the HEMTT workspace system

## Example Usage

```rust
use hemtt_common::config::PDriveOption;
use hemtt_mission::{Mission, ParsedData};
use hemtt_workspace::{LayerType, Workspace};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a workspace for the mission
    let workspace = Workspace::builder()
        .physical(&std::path::PathBuf::from("path/to/mission"), LayerType::Source)
        .finish(None, true, &PDriveOption::Disallow)?;
    
    // Get all mission paths
    let mission_paths = workspace.mission_paths();
    
    // Process the first mission found
    if let Some(mission_path) = mission_paths.first() {
        let mission = Mission::new(mission_path.clone())?;
        
        // Access mission data
        println!("Mission name: {}", mission.name().unwrap_or_else(|| "Unknown".to_string()));
        
        // Access the raw SQM data
        if let Some(sqm_data) = mission.sqm_data() {
            // Work with the raw SqmFile from hemtt-sqm
            if let Some(mission_classes) = sqm_data.classes.get("Mission") {
                println!("Mission class found!");
            }
        }
        
        // Process script files
        for script_file in mission.script_files() {
            println!("Script: {}", script_file.path.filename());
        }
    }
    
    Ok(())
}
```

## Command Line Example

Run the included example:

```
cargo run --example main -- path/to/mission
```

This will:
1. Load the mission folder
2. Parse the mission.sqm file using hemtt-sqm
3. Process all other mission files
4. Display a summary of the mission contents

## Integration with HEMTT

The Mission crate integrates with the HEMTT workspace system, making it easy to process missions within a larger project. It leverages the parsing capabilities of other HEMTT crates to provide a consistent and comprehensive mission processing solution. 