use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use hemtt_sqf::parser::{self, database::Database};
use hemtt_workspace::{LayerType, WorkspacePath, reporting::Processed};
use std::sync::Arc;

const REAL_WORLD_SQF: &str = include_str!("../tests/fixtures/arsenal.sqf");

// Generate synthetic SQF code with different complexities
fn generate_synthetic_sqf(size: usize, complexity: &str) -> String {
    match complexity {
        "simple" => {
            // Simple variable assignments and basic operations
            let mut code = String::with_capacity(size * 20);
            for i in 0..size {
                code.push_str(&format!("_var{} = {} + {};\n", i, i, i + 1));
            }
            code
        }
        "arrays" => {
            // Heavy array operations
            let mut code = String::with_capacity(size * 50);
            code.push_str("_array = [");
            for i in 0..size {
                if i > 0 {
                    code.push_str(", ");
                }
                code.push_str(&i.to_string());
            }
            code.push_str("];\n");
            
            // Array operations
            for i in 0..size {
                code.push_str(&format!("_element{} = _array select {};\n", i, i % size));
            }
            code
        }
        "nested" => {
            // Nested code blocks and control structures
            let mut code = String::with_capacity(size * 100);
            for i in 0..size {
                code.push_str(&format!("if (true) then {{\n"));
                code.push_str(&format!("    _var{} = {};\n", i, i));
                code.push_str(&format!("    if (_var{} > 0) then {{\n", i));
                code.push_str(&format!("        _var{} = _var{} + 1;\n", i, i));
                code.push_str("    };\n");
                code.push_str("};\n");
            }
            code
        }
        "commands" => {
            // Heavy command usage
            let mut code = String::with_capacity(size * 60);
            for i in 0..size {
                code.push_str(&format!("player setPos [_x + {}, _y + {}, 0];\n", i, i));
                code.push_str(&format!("hint str ({} + {});\n", i, i + 1));
                code.push_str("player setDamage 0;\n");
            }
            code
        }
        _ => String::new(),
    }
}

// Process a string directly into a Processed without going through a file
fn process_str(input: &str) -> Processed {
    // Create an in-memory workspace for processing
    let workspace = hemtt_workspace::Workspace::builder()
        .memory()
        .finish(None, false, &hemtt_common::config::PDriveOption::Disallow)
        .unwrap();
    
    // Create a temporary file in the workspace
    let temp_file = workspace.join("temp.sqf").unwrap();
    temp_file.create_file().unwrap().write_all(input.as_bytes()).unwrap();
    
    // Process the file
    hemtt_preprocessor::Processor::run(&temp_file).unwrap()
}

fn bench_real_world(c: &mut Criterion) {
    let mut group = c.benchmark_group("real_world");
    let database = Database::a3(false);
    
    group.bench_function("arsenal_sqf", |b| {
        b.iter(|| {
            let processed = process_str(black_box(REAL_WORLD_SQF));
            parser::run(&database, &processed).unwrap()
        });
    });

    group.finish();
}

fn bench_synthetic(c: &mut Criterion) {
    let mut group = c.benchmark_group("synthetic");
    let database = Database::a3(false);
    
    let sizes = [10, 100, 1000];
    let complexities = ["simple", "arrays", "nested", "commands"];

    for size in sizes {
        for complexity in complexities {
            let sqf_code = generate_synthetic_sqf(size, complexity);
            
            // First validate that the code can be parsed before benchmarking
            let check_processed = process_str(&sqf_code);
            if let Err(err) = parser::run(&database, &check_processed) {
                eprintln!("Failed to parse {} with size {}: {:?}", complexity, size, err);
                continue;
            }
            
            group.bench_with_input(
                BenchmarkId::new(complexity, size),
                &sqf_code,
                |b, sqf| {
                    b.iter(|| {
                        let processed = process_str(black_box(sqf));
                        parser::run(&database, &processed).unwrap()
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(benches, bench_real_world, bench_synthetic);
criterion_main!(benches); 