use std::path::Path;
use std::fs;

fn copy_dir_recursive(src: &Path, dst: &Path) {
    if !src.exists() {
        return;
    }
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            fs::copy(&src_path, &dst_path).unwrap();
        }
    }
}

fn main() {
    // Copy bundled Ghostscript next to the output binary so it works
    // during development (cargo build) without an installer.
    let gs_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("ghostscript");
    
    if !gs_src.exists() {
        println!("cargo:warning=");
        println!("cargo:warning=╔═══════════════════════════════════════════════════════════════════╗");
        println!("cargo:warning=║ Ghostscript bundle not found!                                    ║");
        println!("cargo:warning=║                                                                   ║");
        println!("cargo:warning=║ To bundle Ghostscript with the GUI, run:                         ║");
        println!("cargo:warning=║   .\\scripts\\setup-ghostscript.ps1                                 ║");
        println!("cargo:warning=║                                                                   ║");
        println!("cargo:warning=║ The app will still work if Ghostscript is installed system-wide. ║");
        println!("cargo:warning=╚═══════════════════════════════════════════════════════════════════╝");
        println!("cargo:warning=");
    } else {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        // OUT_DIR is like target/release/build/pdftool-gui-XXX/out
        // We need target/release/ghostscript
        let target_dir = Path::new(&out_dir)
            .ancestors()
            .nth(3)
            .unwrap()
            .to_path_buf();
        let gs_dst = target_dir.join("ghostscript");
        copy_dir_recursive(&gs_src, &gs_dst);
        println!("cargo:warning=✓ Ghostscript bundled to {}", gs_dst.display());
    }

    tauri_build::build()
}
