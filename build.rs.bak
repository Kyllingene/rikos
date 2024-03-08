use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs};

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let kernel = PathBuf::from(std::env::var_os("CARGO_STATICLIB_FILE_KERNEL").unwrap());
    let kernel_path = kernel.parent().unwrap();
    let kernel_name = kernel.file_name().unwrap().to_str().unwrap();
    let kernel_name = kernel_name
        .strip_suffix(".a")
        .unwrap_or(kernel_name)
        .strip_prefix("lib")
        .unwrap_or(kernel_name);

    let mut asm: Vec<PathBuf> = fs::read_dir("kernel/asm")
        .expect("asm directory missing")
        .filter_map(|f| {
            let ext = &OsStr::new("asm");
            f.ok()
                .filter(|f| f.path().extension() == Some(ext))
                .map(|f| f.path())
        })
        .collect();

    for file in asm.iter_mut() {
        let outname = file.file_name().unwrap();
        let mut outpath = out_dir.join(outname);
        outpath.set_extension("o");

        let output = Command::new("nasm")
            .args(["-f", "elf64"])
            .arg(&file)
            .arg("-o")
            .arg(&outpath)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run nasm");

        if !output.status.success() {
            panic!("nasm failed");
        }

        *file = outpath;
    }

    let mut ld = Command::new("ld");
    ld.args(["-n", "--gc-sections", "-o"])
        .arg(out_dir.join("kernel.bin"))
        .arg("-T")
        .arg("kernel/asm/linker.ld")
        .args(asm)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let output = ld
        .arg("-L")
        .arg(kernel_path)
        .args(["-l", kernel_name])
        .output();

    if !output.expect("failed to run ld").status.success() {
        panic!("ld failed");
    }

    fs::copy(out_dir.join("kernel.bin"), "image/boot/kernel.bin")
        .expect("failed to move kernel.bin");

    let output = Command::new("grub-mkrescue")
        .args(["-o", "target/rikos.iso", "image"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run grub-mkrescue");

    if !output.status.success() {
        panic!("grub-mkrescue failed");
    }
}
