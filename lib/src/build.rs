use std::io::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
    .default_package_filename("events.rs")
    .compile_protos(&["src/proto/auth.proto", "src/proto/payment.proto"], &["src/proto/"])?;
    Ok(())
}