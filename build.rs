use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    
    // Compila il kernel esplicitamente prima di creare l'immagine
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let kernel_dir = manifest_dir.join("kernel");
    
    println!("cargo:rerun-if-changed=kernel/src");
    println!("cargo:rerun-if-changed=kernel/Cargo.toml");
    
    // Compila il kernel con il target corretto
    let status = Command::new("cargo")
        .args(&["build", "--release", "--target", "x86-64-lumos.json"])
        .current_dir(&kernel_dir)
        .status()
        .expect("Failed to execute cargo build for kernel");
    
    if !status.success() {
        panic!("Failed to build kernel");
    }
    
    // Percorso corretto: usa la target directory del workspace (root)
    let kernel = manifest_dir  // root del workspace
        .join("target")
        .join("x86-64-lumos")
        .join("release")
        .join("lum-os-kernel");
    
    if !kernel.exists() {
        panic!("Kernel binary not found at: {}", kernel.display());
    }

    // create an UEFI disk image (optional)
    let uefi_path = out_dir.join("uefi.img");
    bootloader::UefiBoot::new(&kernel).create_disk_image(&uefi_path).unwrap();

    // create a BIOS disk image
    let bios_path = out_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel).create_disk_image(&bios_path).unwrap();

    // pass the disk image paths as env variables to the `main.rs`
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}