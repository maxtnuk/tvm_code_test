use anyhow::{Context, Result};
use std::{io::Write, path::Path, process::Command};

fn main() -> Result<()> {
    let out_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let output = Command::new("python")
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/src/build_resnet.py"))
        .arg(&format!("--build-dir={}", env!("CARGO_MANIFEST_DIR")))
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        std::io::stdout()
            .write_all(&output.stderr)
            .context("Failed to write error")?;
        panic!("Failed to execute build script");
    }
    assert!(
        Path::new(&format!("{}/deploy_lib.o", out_dir)).exists(),
        "Could not prepare demo: {}",
        String::from_utf8(output.stderr)
            .unwrap()
            .trim()
            .split("\n")
            .last()
            .unwrap_or("")
    );
    println!("cargo:rustc-link-search=native={}", out_dir);

    Ok(())
}