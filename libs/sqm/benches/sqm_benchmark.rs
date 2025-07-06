use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use hemtt_sqm::{parse_sqm, parse_sqm_with_config, ParallelConfig};
use std::time::Duration;
use std::fs;
use std::path::Path;

fn generate_deep_nested_class(depth: usize, properties_per_level: usize) -> String {
    let mut content = String::new();
    
    // Add some defines at the start
    content.push_str("#define _ARMA_\n");
    content.push_str("#define BENCHMARK\n\n");
    content.push_str("version = 53;\n\n");

    fn generate_level(depth: usize, current_depth: usize, properties: usize) -> String {
        if current_depth >= depth {
            return String::new();
        }

        let mut level = String::new();
        // Add properties
        for i in 0..properties {
            level.push_str(&format!("    prop_{} = {};\n", i, i));
            if i % 2 == 0 {
                level.push_str(&format!("    array_{i}[] = {{1,2,3,4,5}};\n"));
            }
        }

        // Add nested class
        level.push_str(&format!("    class Level_{} {{\n", current_depth));
        level.push_str(&generate_level(depth, current_depth + 1, properties));
        level.push_str("    };\n");
        
        level
    }

    content.push_str("class Root {\n");
    content.push_str(&generate_level(depth, 0, properties_per_level));
    content.push_str("};\n");

    content
}

fn generate_wide_structure(width: usize, properties_per_class: usize) -> String {
    let mut content = String::new();
    
    content.push_str("#define _ARMA_\n");
    content.push_str("#define BENCHMARK_WIDE\n\n");
    content.push_str("version = 53;\n\n");

    for i in 0..width {
        content.push_str(&format!("class Class_{} {{\n", i));
        for j in 0..properties_per_class {
            content.push_str(&format!("    prop_{} = {};\n", j, j));
            if j % 2 == 0 {
                content.push_str(&format!("    array_{j}[] = {{1,2,3,4,5}};\n"));
            }
        }
        content.push_str("};\n\n");
    }

    content
}

fn generate_large_mixed_structure(classes: usize, max_depth: usize, properties_per_class: usize) -> String {
    let mut content = String::new();
    
    content.push_str("#define _ARMA_\n");
    content.push_str("#define BENCHMARK_LARGE\n\n");
    content.push_str("version = 53;\n\n");

    fn generate_class(depth: usize, max_depth: usize, properties: usize, id: &mut usize) -> String {
        let mut class = String::new();
        
        for i in 0..properties {
            class.push_str(&format!("    prop_{} = {};\n", i, i));
            if i % 2 == 0 {
                class.push_str(&format!("    array_{i}[] = {{1,2,3,4,5}};\n"));
            }
        }

        if depth < max_depth && *id % 3 == 0 {
            *id += 1;
            class.push_str(&format!("    class Nested_{} {{\n", id));
            class.push_str(&generate_class(depth + 1, max_depth, properties, id));
            class.push_str("    };\n");
        }

        class
    }

    let mut id = 0;
    for i in 0..classes {
        content.push_str(&format!("class Class_{} {{\n", i));
        content.push_str(&generate_class(0, max_depth, properties_per_class, &mut id));
        content.push_str("};\n\n");
    }

    content
}

fn read_fixture_file(file_path: &str) -> Option<String> {
    fs::read_to_string(Path::new(file_path)).ok()
}

fn generate_fallback_mission(size: &str) -> String {
    match size {
        "small" => generate_deep_nested_class(3, 5),
        "large" => generate_large_mixed_structure(25, 4, 10),
        _ => generate_wide_structure(10, 5),
    }
}

fn bench_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("sqm_parsing");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(30);

    // Test different parallel thresholds
    let thresholds = [100, 200, 500];
    let input = generate_large_mixed_structure(50, 3, 15);
    
    for &threshold in &thresholds {
        let config = ParallelConfig {
            min_tokens_for_parallel: threshold,
            max_parallel_tasks: num_cpus::get(),
            min_properties_for_parallel: 10,
        };
        
        group.bench_with_input(
            BenchmarkId::new("parallel_threshold", threshold),
            &(input.as_str(), config.clone()),
            |b, (input, config)| {
                b.iter(|| parse_sqm_with_config(black_box(input), black_box(config.clone())))
            },
        );
    }

    // Test different structure types
    let cases = [
        ("deep_nested", generate_deep_nested_class(8, 8)),
        ("wide", generate_wide_structure(50, 8)),
        ("mixed", generate_large_mixed_structure(30, 3, 8)),
    ];

    // Test with default config
    for &(name, ref input) in &cases {
        group.bench_with_input(
            BenchmarkId::new("structure_type", name),
            input,
            |b, input| b.iter(|| parse_sqm(black_box(input))),
        );
    }

    group.finish();
}

fn bench_real_world(c: &mut Criterion) {
    let mut group = c.benchmark_group("real_world_sqm");
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(20);
    
    // Load real-world SQM files or use generated fallbacks
    let mission_small = read_fixture_file("tests/fixtures/mission_small.sqm")
        .unwrap_or_else(|| generate_fallback_mission("small"));
    let mission_large = read_fixture_file("tests/fixtures/mission_large.sqm")
        .unwrap_or_else(|| generate_fallback_mission("large"));
    
    // Test with default configuration
    group.bench_with_input(
        BenchmarkId::new("file", "mission_small"),
        &mission_small,
        |b, input| b.iter(|| parse_sqm(black_box(input))),
    );
    
    group.bench_with_input(
        BenchmarkId::new("file", "mission_large"),
        &mission_large,
        |b, input| b.iter(|| parse_sqm(black_box(input))),
    );
    
    // Test with different parallel configurations
    let configs = [
        ("default", ParallelConfig::default()),
        ("aggressive", ParallelConfig {
            min_tokens_for_parallel: 100,
            max_parallel_tasks: num_cpus::get() * 2,
            min_properties_for_parallel: 5,
        }),
        ("conservative", ParallelConfig {
            min_tokens_for_parallel: 500,
            max_parallel_tasks: num_cpus::get(),
            min_properties_for_parallel: 20,
        }),
    ];
    
    for (config_name, config) in &configs {
        group.bench_with_input(
            BenchmarkId::new("mission_small_config", config_name),
            &(&mission_small, config.clone()),
            |b, (input, config)| {
                b.iter(|| parse_sqm_with_config(black_box(input), black_box(config.clone())))
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("mission_large_config", config_name),
            &(&mission_large, config.clone()),
            |b, (input, config)| {
                b.iter(|| parse_sqm_with_config(black_box(input), black_box(config.clone())))
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_parsing, bench_real_world);
criterion_main!(benches); 