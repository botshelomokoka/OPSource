// OPSource library
// Provides Bitcoin functionality with dual Python and Rust implementations
// and Lightning Network functionality using LDK

// Re-export modules
pub mod config;
pub mod bitcoin;
pub mod lightning;

// Initialize all modules
pub fn init() {
    bitcoin::init();
    lightning::init();
}

// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION"); 