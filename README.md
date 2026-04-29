# Social Services Eligibility Engine

A high-precision, modular Rust library and CLI tool designed to determine household eligibility for social service programs based on federal and state-level policy requirements.

This project demonstrates the translation of complex legal mandates (specifically **DHS-managed programs**) into maintainable, type-safe, and high-performance code.

## Overview
Government eligibility programs involve rigid income thresholds and high-stakes financial calculations. This engine utilizes a **Trait-based architecture** to decouple the core data models from specific program logic, ensuring the system is extensible for multiple benefit types (e.g., SNAP, TANF).

## Technical Features
* **Trait-Based Extensibility:** Implements an `EligibilityProgram` trait. New programs can be added without modifying the core `Household` or `Member` structures, adhering to the Open-Closed Principle.
* **High-Precision Arithmetic:** Uses the `rust_decimal` crate to handle financial data, eliminating the rounding risks inherent in floating-point math—a critical requirement for legal compliance.
* **Domain-Specific Data Modeling:** Leverages Rust's `Enum` and `Option` types to represent complex household states such as employment status, able-bodied status, and structured eligibility decisions.
* **Integration Testing:** Features a dedicated suite of tests in `tests/` that validate boundary conditions, ensuring 100% accuracy against 2026 federal income thresholds.

## Project Structure
* **`social-services-eligibility-engine/`**: The core library.
  * `member.rs`: Individual attributes (age, income, employment).
  * `household.rs`: Aggregate calculations for the household unit.
  * `snap.rs`: Supplemental Nutrition Assistance Program implementation.
  * `decision.rs`: Structured feedback on eligibility outcomes.
* **`my_snap_cli/`**: A sample command-line interface demonstrating how to utilize the library.

## Getting Started

### Prerequisites
* Rust
* Cargo

### Installation & Usage
1. Clone the repository:
   ```bash
   git clone https://github.com/wylieglover/social-services-eligibility-engine.git
   ```
2. Run the CLI:
   ```bash
   cd my_snap_cli
   cargo run
   ```
3. Run the library tests:
   ```bash
   cd social-services-eligibility-engine
   cargo test
   ```
## Example Test Case
The engine is built to handle exact boundary cases:
```rust
#[test]
fn test_boundary() {
    let mut household = Household::new();
    // Two members at exactly $1146 each = $2292 (the 2-person limit)
    household.add_member(Member::new("Wylie", 27, FullTime, dec!(1146)));
    household.add_member(Member::new("Franchette", 28, FullTime, dec!(1146)));

    let program = SnapProgram;
    let result = program.eligibility(&household);
    
    assert!(matches!(result, Decision::Eligible { .. }));
}
```

