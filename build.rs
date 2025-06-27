fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR")?;

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(format!("{}/vanessa_descriptor.bin", out_dir))
        .compile_protos(&["proto/vanessa.proto"], &["proto"])?;

    // proto 파일 변경 시 재빌드 트리거
    println!("cargo:rerun-if-changed=proto/vanessa.proto");

    Ok(())
}
