// Internal modules (only Rust uses these)
mod tpmr;
mod proto;
mod config;
mod versionCheck;//tpm version check

// Public modules (Java needs to access them via JNI exports)
pub mod https_client;
pub mod grpc_client;
pub mod bindings;  // JNI calls
