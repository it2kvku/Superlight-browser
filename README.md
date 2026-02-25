# 🛡️ Brownser — Superlight Browser

A **superlight**, native web browser built with **Rust** — weighing only **~0.6 MB**. Features built-in ad blocking (including YouTube ads), persistent sessions, and minimal resource usage.

Built on [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (Windows) via [wry](https://github.com/nicely/nicely) + [tao](https://github.com/nicely/nicely).

---

## ✨ Features

| Feature | Description |
|---|---|
| 🪶 **Superlight** | ~0.6 MB exe, minimal RAM usage |
| 🛡️ **Ad Blocker** | ~200+ blocked ad/tracker domains (EasyList-based) |
| 📺 **YouTube Ad Blocker** | Auto-skips video ads, hides banners & overlays |
| 🔒 **Popup Blocker** | Blocks unwanted popup windows |
| 💾 **Persistent Sessions** | Login stays after restart (cookies saved) |
| 🎨 **Floating Nav Bar** | URL bar + Back/Forward/Reload/Home on every page |
| 🔍 **Smart URL Bar** | Auto-adds `https://`, searches Google for keywords |
| 🚫 **No DevTools** | Disabled for smaller footprint |

## 📦 Download

Go to [**Releases**](https://github.com/it2kvku/Superlight-browser/releases) and download:
- `brownser.exe` — the browser (~0.6 MB)
- `WebView2Loader.dll` — required runtime DLL (~157 KB)

> **Note:** Place both files in the same folder and run `brownser.exe`.

### Requirements
- **Windows 10/11** with [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (pre-installed on most Windows 10/11 machines)

## 🔨 Build from Source

### Prerequisites
- [Rust](https://rustup.rs/) (stable)
- GCC/MinGW (`C:\msys64\mingw64\bin\gcc.exe`) or MSVC Build Tools

### Build

```powershell
# Clone
git clone https://github.com/it2kvku/Superlight-browser.git
cd Superlight-browser

# Build release (optimized, ~0.6 MB)
cargo build --release

# Copy WebView2Loader.dll next to exe
Copy-Item "target\release\build\webview2-com-sys-*\out\x64\WebView2Loader.dll" "target\release\" -Force

# Run
.\target\release\brownser.exe
```

## 🏗️ Project Structure

```
├── Cargo.toml          # Dependencies + size optimizations
├── build.rs            # Auto-copies WebView2Loader.dll, hides console
├── src/
│   ├── main.rs         # Browser window, nav bar, WebView config
│   └── adblocker.rs    # Domain-based ad blocker (~200+ rules)
```

## 🛡️ Ad Blocking

### 3-Layer Protection

1. **Network-level** — Blocks requests to 200+ ad/tracker domains before they load
2. **Cosmetic** — CSS rules hide ad containers that slip through
3. **YouTube-specific** — Auto-clicks "Skip Ad", fast-forwards unskippable ads, hides all banner/overlay/companion ads

### Blocked Categories
- Google Ads, DoubleClick, AdSense
- Facebook/Meta pixel & ads
- Amazon, Twitter/X, Bing ads
- Criteo, Taboola, Outbrain
- 100+ trackers (analytics, heatmaps, etc.)
- YouTube pre-roll, mid-roll, banner ads

## ⚡ Size Optimizations

| Technique | Effect |
|---|---|
| `opt-level = "z"` | Optimize for size |
| `lto = true` | Link-time optimization |
| `codegen-units = 1` | Better whole-program optimization |
| `strip = "symbols"` | Remove debug symbols |
| `panic = "abort"` | Smaller than unwinding |

**Result: 3.53 MB → 0.59 MB (−83%)**

## 📝 License

MIT

---

*Built with ❤️ in Rust*
