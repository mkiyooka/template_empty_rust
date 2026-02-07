// Math module - contains mathematical operations

pub mod advanced;
pub mod ops;

// Re-export commonly used functions for convenience
pub use advanced::power;
pub use ops::{CalcError, add, divide, multiply, subtract};
