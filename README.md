# octofhir-fhir-model

[![Crates.io](https://img.shields.io/crates/v/octofhir-fhir-model.svg)](https://crates.io/crates/octofhir-fhir-model)
[![Documentation](https://docs.rs/octofhir-fhir-model/badge.svg)](https://docs.rs/octofhir-fhir-model)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](#license)

Core FHIR model abstractions and ModelProvider trait for FHIRPath type-aware evaluation.

## Overview

This crate provides the foundational types and traits for FHIR model integration with FHIRPath engines. It serves as an intermediate dependency to break circular dependencies between FHIRPath implementations and FHIR schema libraries.

## Architecture

The crate is designed around the `ModelProvider` trait, which abstracts access to FHIR model information including:

- **Type reflection and metadata** - Introspect FHIR types and their properties
- **Structure definitions and conformance validation** - Validate resources against profiles
- **Constraint definitions and evaluation** - Apply and evaluate FHIR constraints
- **Reference resolution capabilities** - Resolve references between FHIR resources

## Features

- `default` - Core functionality without optional dependencies
- `async` - Enables async support with `async-trait` and `tokio`
- `serde` - Adds serialization support via `serde`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
octofhir-fhir-model = "0.1.0"
```

For async support:

```toml
[dependencies]
octofhir-fhir-model = { version = "0.1.0", features = ["async"] }
```

For serialization support:

```toml
[dependencies]
octofhir-fhir-model = { version = "0.1.0", features = ["serde"] }
```

## Usage

### Basic Usage

```rust
use octofhir_fhir_model::{ModelProvider, TypeReflectionInfo, FhirVersion};

// ModelProvider implementations provide type information
fn example_usage(provider: &dyn ModelProvider) {
    if let Some(type_info) = provider.get_type_reflection("Patient") {
        println!("Patient type: {:?}", type_info);
    }
}
```

### Working with Type Reflection

```rust
use octofhir_fhir_model::{ModelProvider, TypeReflectionInfo};

fn inspect_type(provider: &dyn ModelProvider, type_name: &str) {
    if let Some(reflection) = provider.get_type_reflection(type_name) {
        println!("Type: {}", reflection.type_name);
        println!("Base type: {:?}", reflection.base_type);
        
        for element in &reflection.elements {
            println!("  Element: {} ({})", element.name, element.type_name);
        }
    }
}
```

### Conformance Validation

```rust
use octofhir_fhir_model::{ModelProvider, ConformanceResult};

fn validate_resource(provider: &dyn ModelProvider, resource_data: &str) -> ConformanceResult {
    // Implementation would validate the resource against FHIR profiles
    // This is typically implemented by concrete ModelProvider implementations
    provider.validate_conformance(resource_data)
}
```

## Modules

- **`boxing`** - Boxed value types and extensions for FHIRPath evaluation
- **`conformance`** - Conformance validation results and violation reporting
- **`constraints`** - Constraint definitions and evaluation results
- **`error`** - Error types and result handling
- **`provider`** - Core ModelProvider trait and related types
- **`reflection`** - Type reflection and metadata structures

## FHIR Version Support

This crate is designed to work with multiple FHIR versions through the `FhirVersion` enum:

- FHIR R4 (4.0.1)
- FHIR R4B (4.3.0)
- FHIR R5 (5.0.0)

## Development

### Prerequisites

- Rust 2024 edition or later
- [just](https://github.com/casey/just) command runner (optional, for convenience)

### Building

```bash
cargo build
```

Or using just:

```bash
just build
```

### Testing

```bash
cargo test --all-features
```

Or using just:

```bash
just test-all
```

### Documentation

Generate and open documentation:

```bash
cargo doc --no-deps --all-features --open
```

Or using just:

```bash
just doc-open
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run the test suite: `just test-all`
5. Run formatting and linting: `just fmt` and `just lint`
6. Submit a pull request

## License

This project is dual-licensed under either:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgments

This project is part of the [OctoFHIR](https://github.com/octofhir) ecosystem for FHIR processing in Rust.
