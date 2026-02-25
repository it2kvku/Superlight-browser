use std::path::Path;

fn main() {
    // Hide the console window on Windows
    #[cfg(target_os = "windows")]
    {
        // For GNU toolchain (MinGW) - hide console
        println!("cargo:rustc-link-arg=-mwindows");

        // Auto-copy WebView2Loader.dll to the output directory
        let out_dir = std::env::var("OUT_DIR").unwrap();
        // Navigate from OUT_DIR (target/{profile}/build/{crate}-{hash}/out)
        // to the target/{profile}/ directory
        let out_path = Path::new(&out_dir);
        if let Some(build_dir) = out_path.parent().and_then(|p| p.parent()).and_then(|p| p.parent()) {
            let profile_dir = build_dir; // target/{profile}/build -> target/{profile}

            // Find webview2-com-sys build output
            let build_path = profile_dir.join("build");
            if let Ok(entries) = std::fs::read_dir(&build_path) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("webview2-com-sys-") {
                        let dll_src = entry.path().join("out").join("x64").join("WebView2Loader.dll");
                        let dll_dst = profile_dir.join("WebView2Loader.dll");
                        if dll_src.exists() && !dll_dst.exists() {
                            let _ = std::fs::copy(&dll_src, &dll_dst);
                            eprintln!("Copied WebView2Loader.dll to {:?}", dll_dst);
                        }
                    }
                }
            }
        }
    }
}
