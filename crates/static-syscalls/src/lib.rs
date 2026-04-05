pub mod rent;
pub mod hash;
pub mod clock;

// Re-export for convenience
pub use clock::StaticClock;
pub use rent::StaticRent;