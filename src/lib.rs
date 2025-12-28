/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 25/12/25
******************************************************************************/

//! # Positive
//!
//! A type-safe wrapper for guaranteed positive decimal values in Rust.
//!
//! ## Overview
//!
//! `Positive` is a Rust library that provides a type-safe wrapper around `Decimal` values,
//! ensuring that the contained value is always non-negative (>= 0). This is particularly
//! useful in financial applications where negative values would be invalid or meaningless,
//! such as prices, quantities, volatilities, and other strictly positive metrics.
//!
//! ## Features
//!
//! - **Type Safety**: Compile-time and runtime guarantees that values are non-negative
//! - **Decimal Precision**: Built on [`rust_decimal`](https://crates.io/crates/rust_decimal) for accurate financial calculations
//! - **Rich API**: Comprehensive arithmetic operations, conversions, and mathematical utilities
//! - **Serde Support**: Full serialization/deserialization support for JSON and other formats
//! - **Approx Support**: Approximate equality comparisons for floating-point tolerance
//! - **Checked Operations**: Safe arithmetic operations that return `Result` instead of panicking
//! - **Optional utoipa Integration**: OpenAPI schema generation support via feature flag
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! positive = "0.1"
//! ```
//!
//! To enable OpenAPI schema support:
//!
//! ```toml
//! [dependencies]
//! positive = { version = "0.1", features = ["utoipa"] }
//! ```
//!
//! ## Quick Start
//!
//! ```rust
//! use positive::{Positive, pos, pos_or_panic};
//!
//! // Create a positive value using the macro (returns Result)
//! let price = pos!(100.50).unwrap();
//!
//! // Or use pos_or_panic! for direct value (panics on invalid input)
//! let price = pos_or_panic!(100.50);
//!
//! // Create using the constructor
//! let quantity = Positive::new(10.0).unwrap();
//!
//! // Arithmetic operations
//! let total = price * quantity;
//!
//! // Safe operations that return Result
//! let discount = pos_or_panic!(5.0);
//! let final_price = price.checked_sub(&discount).unwrap();
//!
//! // Saturating subtraction (returns ZERO instead of negative)
//! let result = pos_or_panic!(3.0).saturating_sub(&pos_or_panic!(5.0));
//! assert_eq!(result, Positive::ZERO);
//! ```
//!
//! ## API Overview
//!
//! ### Creation
//!
//! ```rust
//! use positive::{Positive, pos, pos_or_panic, spos};
//! use rust_decimal::Decimal;
//!
//! // From f64
//! let p = Positive::new(5.0).unwrap();
//!
//! // From Decimal
//! let p = Positive::new_decimal(Decimal::ONE).unwrap();
//!
//! // Using macros
//! let p = pos!(5.0);           // Returns Result<Positive, PositiveError>
//! let p = pos_or_panic!(5.0);  // Panics on invalid input
//! let p = spos!(5.0);          // Returns Option<Positive>
//! ```
//!
//! ### Constants
//!
//! ```rust
//! use positive::Positive;
//!
//! let zero = Positive::ZERO;       // 0
//! let one = Positive::ONE;         // 1
//! let two = Positive::TWO;         // 2
//! let ten = Positive::TEN;         // 10
//! let hundred = Positive::HUNDRED; // 100
//! let thousand = Positive::THOUSAND; // 1000
//! let pi = Positive::PI;           // π
//! let inf = Positive::INFINITY;    // Maximum value
//! ```
//!
//! ### Conversions
//!
//! ```rust
//! use positive::pos_or_panic;
//!
//! let p = pos_or_panic!(5.5);
//!
//! let f: f64 = p.to_f64();              // Panics on failure
//! let f: Option<f64> = p.to_f64_checked(); // Returns None on failure
//! let f: f64 = p.to_f64_lossy();        // Returns 0.0 on failure
//! let i: i64 = p.to_i64();              // To signed integer
//! let u: u64 = p.to_u64();              // To unsigned integer
//! let d = p.to_dec();                   // To Decimal
//! ```
//!
//! ### Arithmetic Operations
//!
//! ```rust
//! use positive::pos_or_panic;
//!
//! let a = pos_or_panic!(10.0);
//! let b = pos_or_panic!(3.0);
//!
//! // Standard operations
//! let sum = a + b;        // Addition
//! let diff = a - b;       // Subtraction (panics if result < 0)
//! let prod = a * b;       // Multiplication
//! let quot = a / b;       // Division
//!
//! // Safe operations
//! let safe_diff = a.checked_sub(&b);    // Returns Result
//! let sat_diff = a.saturating_sub(&b);  // Returns ZERO if result < 0
//! let safe_quot = a.checked_div(&b);    // Returns Result (handles div by zero)
//! ```
//!
//! ### Mathematical Functions
//!
//! ```rust
//! use positive::pos_or_panic;
//!
//! let p = pos_or_panic!(16.0);
//!
//! let sqrt = p.sqrt();           // Square root
//! let ln = p.ln();               // Natural logarithm
//! let log10 = p.log10();         // Base-10 logarithm
//! let exp = p.exp();             // Exponential (e^x)
//! let pow = p.pow(pos_or_panic!(2.0));    // Power with Positive exponent
//! let powi = p.powi(2);          // Integer power
//! let floor = p.floor();         // Floor
//! let ceil = p.ceiling();        // Ceiling
//! let round = p.round();         // Round to nearest integer
//! let round2 = p.round_to(2);    // Round to 2 decimal places
//! ```
//!
//! ### Utility Methods
//!
//! ```rust
//! use positive::pos_or_panic;
//!
//! let p = pos_or_panic!(5.0);
//!
//! let is_zero = p.is_zero();                      // Check if zero
//! let is_mult = p.is_multiple(2.0);               // Check if multiple of value
//! let clamped = p.clamp(pos_or_panic!(1.0), pos_or_panic!(10.0));   // Clamp between bounds
//! let min_val = p.min(pos_or_panic!(3.0));                 // Minimum of two values
//! let max_val = p.max(pos_or_panic!(3.0));                 // Maximum of two values
//! let formatted = p.format_fixed_places(2);       // Format with fixed decimals
//! ```
//!
//! ## Error Handling
//!
//! The library provides `PositiveError` for comprehensive error handling:
//!
//! ```rust
//! use positive::{Positive, PositiveError};
//!
//! fn example() -> Result<Positive, PositiveError> {
//!     let value = Positive::new(-5.0)?;  // Returns Err(OutOfBounds)
//!     Ok(value)
//! }
//! ```
//!
//! Error variants include:
//! - `InvalidValue` - Value cannot be represented as a valid positive decimal
//! - `ArithmeticError` - Error during mathematical operations
//! - `ConversionError` - Error when converting between types
//! - `OutOfBounds` - Value exceeds defined limits
//! - `InvalidPrecision` - Invalid decimal precision settings
//!
//! ## Serialization
//!
//! `Positive` implements `Serialize` and `Deserialize`:
//!
//! ```rust
//! use positive::pos_or_panic;
//!
//! let p = pos_or_panic!(42.5);
//! let json = serde_json::to_string(&p).unwrap();  // "42.5"
//! let parsed: positive::Positive = serde_json::from_str(&json).unwrap();
//! ```
//!
//! ## Use Cases
//!
//! - **Financial Applications**: Prices, quantities, fees, rates
//! - **Scientific Computing**: Physical quantities that cannot be negative
//! - **Game Development**: Health points, distances, timers
//! - **Data Validation**: Ensuring input values meet positivity constraints
//!
//! ## License
//!
//! This project is licensed under the MIT License.
//!

pub mod error;
mod positive;
mod tests;
pub use error::{PositiveError, PositiveResult};
pub use positive::*;

/// Re-export rust_decimal for convenience.
pub use rust_decimal::Decimal;
