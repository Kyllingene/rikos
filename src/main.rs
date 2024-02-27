#[cfg(debug_assertions)]
fn main() {
    use std::process::{Command, Stdio};

    let output = Command::new("qemu-system-x86_64")
        .args([
            "-drive",
            "format=raw,file=target/rikos.iso",
            "-serial",
            "stdio",
            "-s", //"-S",
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

#[cfg(not(debug_assertions))]
compile_error!("run `cargo build --release` instead of `cargo run --release`");
