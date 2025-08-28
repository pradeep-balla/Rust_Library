// Internal modules (only Rust uses these)
mod tpmr;
mod proto;
mod config;

// Public modules (Java needs to access them via JNI exports)
pub mod https_client;
pub mod grpc_client;
pub mod bindings;  // JNI calls
