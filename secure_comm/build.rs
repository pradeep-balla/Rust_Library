fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use vendored protoc to avoid system dependency, especially on Windows
    let protoc_path = protoc_bin_vendored::protoc_bin_path()?;
    std::env::set_var("PROTOC", protoc_path);

    tonic_build::configure()
        .compile(&["proto/verifier.proto"], &["proto"]) ?;
    Ok(())
}
