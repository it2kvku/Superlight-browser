use std::path::Path;
use std::io::Write;

fn main() {
    #[cfg(target_os = "windows")]
    {
        // Hide the console window (GNU/MinGW toolchain)
        println!("cargo:rustc-link-arg=-mwindows");
    }

    // === Create extensions.zip at build time ===
    let extensions_dir = Path::new("extensions");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let zip_path = Path::new(&out_dir).join("extensions.zip");

    if extensions_dir.exists() {
        let file = std::fs::File::create(&zip_path).expect("Failed to create extensions.zip");
        let mut zip_writer = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .compression_level(Some(9)); // max compression

        for entry in walkdir::WalkDir::new(extensions_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            let name = path.strip_prefix(".").unwrap_or(path);
            let name_str = name.to_string_lossy().replace('\\', "/");

            if path.is_file() {
                zip_writer.start_file(&name_str, options).unwrap();
                let data = std::fs::read(path).unwrap();
                zip_writer.write_all(&data).unwrap();
            } else if path.is_dir() && path != extensions_dir {
                let dir_name = format!("{}/", name_str);
                zip_writer.add_directory(&dir_name, options).unwrap();
            }
        }
        zip_writer.finish().unwrap();
        eprintln!("Created extensions.zip ({} bytes)", std::fs::metadata(&zip_path).unwrap().len());
    }

    // === Copy WebView2Loader.dll to output (for non-embedded builds) ===
    #[cfg(target_os = "windows")]
    {
        let out_path = Path::new(&out_dir);
        // Also save DLL path so main.rs can embed it
        if let Some(build_dir) = out_path.parent().and_then(|p| p.parent()).and_then(|p| p.parent()) {
            let build_path = build_dir.join("build");
            if let Ok(entries) = std::fs::read_dir(&build_path) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("webview2-com-sys-") {
                        let dll_src = entry.path().join("out").join("x64").join("WebView2Loader.dll");
                        if dll_src.exists() {
                            // Copy to OUT_DIR so include_bytes! can find it
                            let dll_dst = Path::new(&out_dir).join("WebView2Loader.dll");
                            let _ = std::fs::copy(&dll_src, &dll_dst);
                            eprintln!("Copied WebView2Loader.dll to {:?}", dll_dst);
                            break;
                        }
                    }
                }
            }
        }
    }

    // Re-run build script if extensions change
    println!("cargo:rerun-if-changed=extensions");
}
