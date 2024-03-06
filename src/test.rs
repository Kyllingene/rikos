use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("qemu-system-x86_64")
        .args([
            "-drive",
            "format=raw,file=target/rikos.iso",
            "-serial",
            "stdio",
            // "-display", "none",
        ])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run qemu");

    if !output.status.success() {
        eprintln!("qemu failed with exit code {}", output.status);
    }
}
