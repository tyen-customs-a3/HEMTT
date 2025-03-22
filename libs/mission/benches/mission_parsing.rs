use std::path::PathBuf;
use std::sync::Arc;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use hemtt_mission::{Mission, scan_and_print_diagnostics, Error, files::{FileType, MissionFile}};
use hemtt_workspace::{WorkspacePath, reporting::{Codes, Code}};
use rayon::prelude::*;

fn parse_mission(path: &str) -> Mission {
    let path_buf = PathBuf::from(path);
    let workspace_path = WorkspacePath::slim(&path_buf).unwrap();
    Mission::new(workspace_path).unwrap()
}

fn parse_mission_with_diagnostics(path: &str) {
    scan_and_print_diagnostics(path).unwrap();
}

// Manual implementation of a sequential version for comparison
fn parse_files_sequential(file_type: FileType, paths: &[WorkspacePath]) -> Vec<Result<MissionFile, Error>> {
    paths.iter()
        .map(|path| hemtt_mission::parser::parse_file(file_type, path.clone()))
        .collect()
}

// Parallel version using rayon
fn parse_files_parallel(file_type: FileType, paths: &[WorkspacePath]) -> Vec<Result<MissionFile, Error>> {
    paths.par_iter()
        .map(|path| hemtt_mission::parser::parse_file(file_type, path.clone()))
        .collect()
}

fn parallel_vs_sequential_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parallel vs Sequential");
    group.sample_size(10);  // Reduce sample count to 10
    
    // Parse each mission once to get file lists
    for (name, path) in [
        ("guerilla_raid", "tests/co22_guerilla_raid.Altis"),
        ("joust", "tests/adv48_Joust.VR"),
    ] {
        let path_buf = PathBuf::from(path);
        let workspace_path = WorkspacePath::slim(&path_buf).unwrap();
        
        // Get list of SQF files
        if let Ok(scanned_files) = hemtt_mission::parser::scan_files(&workspace_path) {
            if let Some(sqf_files) = scanned_files.get(&FileType::Script) {
                if !sqf_files.is_empty() {
                    // Benchmark sequential parsing
                    group.bench_function(
                        BenchmarkId::new("sequential_sqf", name),
                        |b| b.iter(|| parse_files_sequential(
                            FileType::Script, 
                            black_box(sqf_files)
                        ))
                    );
                    
                    // Benchmark parallel parsing
                    group.bench_function(
                        BenchmarkId::new("parallel_sqf", name),
                        |b| b.iter(|| parse_files_parallel(
                            FileType::Script, 
                            black_box(sqf_files)
                        ))
                    );
                }
            }
            
            // Same for config files
            if let Some(config_files) = scanned_files.get(&FileType::Config(hemtt_mission::ConfigFileType::Ext)) {
                if !config_files.is_empty() {
                    group.bench_function(
                        BenchmarkId::new("sequential_config", name),
                        |b| b.iter(|| parse_files_sequential(
                            FileType::Config(hemtt_mission::ConfigFileType::Ext), 
                            black_box(config_files)
                        ))
                    );
                    
                    group.bench_function(
                        BenchmarkId::new("parallel_config", name),
                        |b| b.iter(|| parse_files_parallel(
                            FileType::Config(hemtt_mission::ConfigFileType::Ext), 
                            black_box(config_files)
                        ))
                    );
                }
            }
        }
    }
    
    group.finish();
}

fn mission_parsing_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Mission Parsing");
    group.sample_size(10);  // Reduce sample count to 10
    
    // Test missions
    let missions = [
        ("guerilla_raid", "tests/co22_guerilla_raid.Altis"),
        ("joust", "tests/adv48_Joust.VR"),
    ];

    // Benchmark basic parsing
    for (name, path) in missions.iter() {
        group.bench_with_input(
            BenchmarkId::new("parse", name), 
            path,
            |b, path| b.iter(|| parse_mission(black_box(path)))
        );
    }

    // Benchmark parsing with full diagnostics
    for (name, path) in missions.iter() {
        group.bench_with_input(
            BenchmarkId::new("parse_with_diagnostics", name),
            path,
            |b, path| b.iter(|| parse_mission_with_diagnostics(black_box(path)))
        );
    }

    group.finish();
}

fn file_type_parsing_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("File Type Parsing");
    group.sample_size(10);  // Reduce sample count to 10
    
    // Parse each mission once to analyze file types
    for (name, path) in [
        ("guerilla_raid", "tests/co22_guerilla_raid.Altis"),
        ("joust", "tests/adv48_Joust.VR"),
    ] {
        let mission = parse_mission(path);
        
        // Benchmark SQF file parsing
        let sqf_files = mission.files_of_type(hemtt_mission::FileType::Script);
        if !sqf_files.is_empty() {
            group.bench_function(
                BenchmarkId::new("sqf_files", name),
                |b| b.iter(|| {
                    for file in black_box(sqf_files) {
                        black_box(file.script_data().unwrap());
                    }
                })
            );
        }

        // Benchmark config file parsing
        let config_files = mission.files_of_type(hemtt_mission::FileType::Config(hemtt_mission::ConfigFileType::Ext));
        if !config_files.is_empty() {
            group.bench_function(
                BenchmarkId::new("config_files", name),
                |b| b.iter(|| {
                    for file in black_box(config_files) {
                        black_box(file.config_data().unwrap());
                    }
                })
            );
        }

        // Benchmark mission.sqm parsing
        if let Some(sqm_file) = mission.sqm() {
            group.bench_function(
                BenchmarkId::new("mission_sqm", name),
                |b| b.iter(|| {
                    black_box(sqm_file.sqm_data().unwrap());
                })
            );
        }
    }

    group.finish();
}

fn mission_analysis_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Mission Analysis");
    group.sample_size(10);  // Reduce sample count to 10
    
    for (name, path) in [
        ("guerilla_raid", "tests/co22_guerilla_raid.Altis"),
        ("joust", "tests/adv48_Joust.VR"),
    ] {
        let mission = parse_mission(path);
        
        // Benchmark error checking
        group.bench_function(
            BenchmarkId::new("error_check", name),
            |b| b.iter(|| {
                black_box(hemtt_mission::has_parsing_errors(&mission));
                black_box(hemtt_mission::get_error_counts(&mission));
            })
        );

        // Benchmark metadata extraction
        group.bench_function(
            BenchmarkId::new("metadata", name),
            |b| b.iter(|| {
                black_box(mission.name());
                black_box(mission.author());
            })
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    mission_parsing_benchmark,
    file_type_parsing_benchmark,
    mission_analysis_benchmark,
    parallel_vs_sequential_benchmark
);
criterion_main!(benches); 