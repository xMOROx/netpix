use std::io::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
        .out_dir("src/gen")
        .compile_protos(&["src/proto/rtc_event_log2.proto"], &["src/"])?;
    Ok(())
}
