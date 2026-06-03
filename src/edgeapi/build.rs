// src/edge-api-rs/build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc_path = protoc_bin_vendored::protoc_bin_path()?;

    unsafe {
        std::env::set_var("PROTOC", protoc_path);
    }

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&["proto/demo.proto"], &["proto"])?;

    Ok(())
}