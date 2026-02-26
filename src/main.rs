mod adblocker;

use adblocker::AdBlocker;
use std::sync::{Arc, atomic::{AtomicU32, Ordering}};
use std::path::PathBuf;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{WebViewBuilder, NewWindowResponse};

#[cfg(windows)]
use wry::WebViewBuilderExtWindows;

const HOME_URL: &str = "https://www.google.com";

/// JavaScript injected into every page to create a floating nav bar overlay
const NAV_BAR_JS: &str = r#"
(function() {
    'use strict';

    // Don't inject into iframes
    if (window.self !== window.top) return;

    // Prevent double injection
    if (document.getElementById('brownser-nav')) return;

    const nav = document.createElement('div');
    nav.id = 'brownser-nav';
    nav.innerHTML = `
        <style>
            #brownser-nav {
                position: fixed !important;
                top: 0 !important;
                left: 0 !important;
                right: 0 !important;
                height: 42px !important;
                background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%) !important;
                display: flex !important;
                align-items: center !important;
                gap: 5px !important;
                padding: 0 10px !important;
                z-index: 2147483647 !important;
                font-family: 'Segoe UI', system-ui, sans-serif !important;
                box-shadow: 0 2px 8px rgba(0,0,0,0.4) !important;
                border-bottom: 1px solid rgba(255,255,255,0.06) !important;
            }
            #brownser-nav * {
                box-sizing: border-box !important;
            }
            #brownser-nav .bn-btn {
                width: 30px !important;
                height: 30px !important;
                border: none !important;
                border-radius: 6px !important;
                background: rgba(255,255,255,0.07) !important;
                color: #a0a0b8 !important;
                font-size: 14px !important;
                cursor: pointer !important;
                display: flex !important;
                align-items: center !important;
                justify-content: center !important;
                transition: all 0.12s ease !important;
                flex-shrink: 0 !important;
                padding: 0 !important;
                margin: 0 !important;
                line-height: 1 !important;
            }
            #brownser-nav .bn-btn:hover {
                background: rgba(255,255,255,0.14) !important;
                color: #fff !important;
            }
            #brownser-nav .bn-btn:active {
                transform: scale(0.92) !important;
            }
            #brownser-nav .bn-url {
                flex: 1 !important;
                height: 30px !important;
                border: 1px solid rgba(255,255,255,0.08) !important;
                border-radius: 8px !important;
                background: rgba(255,255,255,0.05) !important;
                color: #e0e0e0 !important;
                padding: 0 12px !important;
                font-size: 12.5px !important;
                font-family: 'Segoe UI', system-ui, sans-serif !important;
                outline: none !important;
                min-width: 0 !important;
            }
            #brownser-nav .bn-url:focus {
                border-color: rgba(100,140,255,0.5) !important;
                background: rgba(255,255,255,0.09) !important;
                box-shadow: 0 0 0 2px rgba(100,140,255,0.1) !important;
            }
            #brownser-nav .bn-shield {
                color: #4ade80 !important;
                font-size: 13px !important;
                padding: 0 4px !important;
                white-space: nowrap !important;
            }
        </style>
        <button class="bn-btn" id="bn-back" title="Back">&#8592;</button>
        <button class="bn-btn" id="bn-fwd" title="Forward">&#8594;</button>
        <button class="bn-btn" id="bn-reload" title="Reload">&#8635;</button>
        <button class="bn-btn" id="bn-home" title="Home">&#127968;</button>
        <input type="text" class="bn-url" id="bn-url" spellcheck="false" autocomplete="off">
        <span class="bn-shield" id="bn-shield" title="Ads blocked">&#128737;</span>
    `;

    function injectNav() {
        if (document.getElementById('brownser-nav')) return;
        const target = document.body || document.documentElement;
        if (!target) return;
        target.appendChild(nav);

        // Push page content down so nav doesn't overlap
        document.documentElement.style.setProperty('margin-top', '42px', 'important');

        const urlBar = document.getElementById('bn-url');
        urlBar.value = window.location.href;

        urlBar.addEventListener('keydown', function(e) {
            if (e.key === 'Enter') {
                e.preventDefault();
                let url = urlBar.value.trim();
                if (!url) return;
                if (!url.includes('.') || url.includes(' ')) {
                    url = 'https://www.google.com/search?q=' + encodeURIComponent(url);
                } else if (!url.startsWith('http://') && !url.startsWith('https://')) {
                    url = 'https://' + url;
                }
                window.location.href = url;
            }
        });

        urlBar.addEventListener('focus', function() {
            setTimeout(() => urlBar.select(), 50);
        });

        document.getElementById('bn-back').addEventListener('click', () => history.back());
        document.getElementById('bn-fwd').addEventListener('click', () => history.forward());
        document.getElementById('bn-reload').addEventListener('click', () => location.reload());
        document.getElementById('bn-home').addEventListener('click', () => {
            window.location.href = 'https://www.google.com';
        });
    }

    if (document.body) {
        injectNav();
    } else {
        document.addEventListener('DOMContentLoaded', injectNav);
    }
})();
"#;

/// JavaScript for cosmetic ad hiding
const COSMETIC_BLOCKER_JS: &str = r#"
(function() {
    'use strict';
    const style = document.createElement('style');
    style.textContent = `
        [class*="ad-container"],
        [class*="ad-wrapper"],
        [class*="ad-banner"],
        [class*="adsbygoogle"],
        [id*="google_ads"],
        [id*="ad-container"],
        [id*="ad-wrapper"],
        iframe[src*="doubleclick"],
        iframe[src*="googlesyndication"],
        iframe[src*="ads"],
        div[data-ad],
        div[data-adunit],
        ins.adsbygoogle,
        div.ad-slot,
        div.advertisement,
        div.sponsored-content {
            display: none !important;
            height: 0 !important;
            min-height: 0 !important;
            max-height: 0 !important;
            overflow: hidden !important;
            visibility: hidden !important;
        }
    `;
    if (document.head) {
        document.head.appendChild(style);
    } else {
        document.addEventListener('DOMContentLoaded', function() {
            document.head.appendChild(style);
        });
    }
})();
"#;

/// JavaScript for YouTube-specific ad blocking — skips video ads, hides overlays
const YOUTUBE_AD_BLOCKER_JS: &str = r#"
(function() {
    'use strict';

    // Only run on YouTube
    if (!location.hostname.includes('youtube.com')) return;

    // Don't inject into iframes
    if (window.self !== window.top) return;

    // === CSS: Hide YouTube ad UI elements ===
    const style = document.createElement('style');
    style.textContent = `
        /* Video ad overlays */
        .ad-showing .video-ads,
        .ad-showing .ytp-ad-module,
        .ytp-ad-overlay-container,
        .ytp-ad-overlay-slot,
        .ytp-ad-image-overlay,
        .ytp-ad-text-overlay,
        .ytp-ad-skip-ad-slot,

        /* Banner ads in feed */
        ytd-banner-promo-renderer,
        ytd-statement-banner-renderer,
        ytd-in-feed-ad-layout-renderer,
        ytd-ad-slot-renderer,
        ytd-rich-item-renderer:has(ytd-ad-slot-renderer),
        ytd-display-ad-renderer,
        ytd-promoted-sparkles-web-renderer,
        ytd-promoted-sparkles-text-search-renderer,
        ytd-player-legacy-desktop-watch-ads-renderer,
        ytd-compact-promoted-item-renderer,
        #player-ads,
        #masthead-ad,
        #panels .ytd-ads-engagement-panel-content-renderer,
        tp-yt-paper-dialog:has(ytd-enforcement-message-view-model),

        /* Companion ads (sidebar) */
        ytd-companion-slot-renderer,
        .ytd-companion-slot-renderer,
        #companion,
        .ytd-merch-shelf-renderer,
        ytd-merch-shelf-renderer,

        /* Popup ads */
        .ytp-ad-action-interstitial,
        .ytp-ad-action-interstitial-background-container,
        .ytp-ad-action-interstitial-slot,
        .ytp-ad-image-overlay,

        /* Premium upsell */
        ytd-mealbar-promo-renderer,
        .ytd-popup-container:has(a[href*="premium"]),
        tp-yt-paper-dialog:has(yt-mealbar-promo-renderer),
        ytmusic-mealbar-promo-renderer,

        /* Misc ad elements */
        .sparkles-light-cta,
        .badge-style-type-ad,
        .google-revocation-link-element,
        .ytp-ad-progress-list {
            display: none !important;
        }

        /* Fix video player after ad removal */
        .ad-showing .html5-video-container {
            display: block !important;
        }
        .ad-showing video {
            visibility: visible !important;
        }
    `;
    document.documentElement.appendChild(style);

    // === Auto-skip video ads ===
    let skipAttempts = 0;
    const maxSkipAttempts = 100;

    function trySkipAd() {
        // 1. Click "Skip Ad" button (multiple selectors for different YT versions)
        const skipSelectors = [
            '.ytp-ad-skip-button',
            '.ytp-ad-skip-button-modern',
            '.ytp-skip-ad-button',
            'button.ytp-ad-skip-button-modern',
            '.ytp-ad-skip-button-container button',
            '[id^="skip-button"]',
            '.ytp-ad-skip-button-text',
        ];
        for (const sel of skipSelectors) {
            const btn = document.querySelector(sel);
            if (btn && btn.offsetParent !== null) {
                btn.click();
                return true;
            }
        }

        // 2. Fast-forward unskippable ads
        const player = document.querySelector('.ad-showing video');
        if (player && player.duration && player.duration > 0 && isFinite(player.duration)) {
            player.currentTime = player.duration;
            player.muted = true;
            player.playbackRate = 16; // Max speed
            return true;
        }

        return false;
    }

    function checkAndSkip() {
        const adShowing = document.querySelector('.ad-showing');
        if (adShowing) {
            trySkipAd();
            skipAttempts++;
            if (skipAttempts < maxSkipAttempts) {
                setTimeout(checkAndSkip, 100);
            }
        } else {
            skipAttempts = 0;
        }
    }

    // === MutationObserver: detect when ads appear ===
    const observer = new MutationObserver(function(mutations) {
        for (const mutation of mutations) {
            // Check if ad-showing class was added
            const target = mutation.target;
            if (target.classList && target.classList.contains('ad-showing')) {
                skipAttempts = 0;
                checkAndSkip();
                return;
            }

            // Check added nodes for ad elements
            for (const node of mutation.addedNodes) {
                if (node.nodeType !== 1) continue;
                if (node.classList && (
                    node.classList.contains('ytp-ad-module') ||
                    node.classList.contains('ytp-ad-overlay-container') ||
                    node.classList.contains('ad-showing')
                )) {
                    skipAttempts = 0;
                    checkAndSkip();
                    return;
                }
                // Check for ad slot renderers
                if (node.tagName && (
                    node.tagName.toLowerCase() === 'ytd-ad-slot-renderer' ||
                    node.tagName.toLowerCase() === 'ytd-in-feed-ad-layout-renderer' ||
                    node.tagName.toLowerCase() === 'ytd-banner-promo-renderer'
                )) {
                    node.remove();
                    return;
                }
            }
        }
    });

    observer.observe(document.documentElement, {
        childList: true,
        subtree: true,
        attributes: true,
        attributeFilter: ['class']
    });

    // === Periodic check (fallback) ===
    setInterval(function() {
        // Remove ad containers that slip through
        const adElements = document.querySelectorAll(
            'ytd-ad-slot-renderer, ytd-in-feed-ad-layout-renderer, ' +
            'ytd-banner-promo-renderer, ytd-promoted-sparkles-web-renderer, ' +
            'ytd-display-ad-renderer, ytd-compact-promoted-item-renderer'
        );
        adElements.forEach(el => el.remove());

        // Check for active video ads
        if (document.querySelector('.ad-showing')) {
            trySkipAd();
        }
    }, 1000);

    // Immediately check on load
    if (document.readyState === 'complete') {
        checkAndSkip();
    } else {
        window.addEventListener('load', checkAndSkip);
    }

    console.log('[Brownser] YouTube ad blocker active');
})();
"#;

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Brownser")
        .with_inner_size(tao::dpi::LogicalSize::new(1200.0, 800.0))
        .with_decorations(true)
        .build(&event_loop)
        .unwrap();

    let ad_blocker = Arc::new(AdBlocker::new());
    let blocked_count = Arc::new(AtomicU32::new(0));

    let ad_blocker_nav = ad_blocker.clone();
    let blocked_count_nav = blocked_count.clone();

    // === EMBEDDED ASSETS: extensions are compiled into the exe ===
    let app_dir = {
        let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(appdata).join("Brownser")
    };
    let _ = std::fs::create_dir_all(&app_dir);

    // Extract extensions from embedded zip to AppData
    let extensions_dir = app_dir.join("extensions");
    let version_file = extensions_dir.join(".version");
    let current_version = env!("CARGO_PKG_VERSION");
    let needs_extract = if version_file.exists() {
        std::fs::read_to_string(&version_file).unwrap_or_default().trim() != current_version
    } else {
        true
    };

    if needs_extract {
        static EMBEDDED_ZIP: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/extensions.zip"));
        eprintln!("[EMBED] Extracting extensions ({} bytes compressed)...", EMBEDDED_ZIP.len());
        let cursor = std::io::Cursor::new(EMBEDDED_ZIP);
        if let Ok(mut archive) = zip::ZipArchive::new(cursor) {
            // Clean old extensions
            let _ = std::fs::remove_dir_all(&extensions_dir);
            let _ = std::fs::create_dir_all(&extensions_dir);
            for i in 0..archive.len() {
                if let Ok(mut file) = archive.by_index(i) {
                    let out_path = app_dir.join(file.name());
                    if file.is_dir() {
                        let _ = std::fs::create_dir_all(&out_path);
                    } else {
                        if let Some(parent) = out_path.parent() {
                            let _ = std::fs::create_dir_all(parent);
                        }
                        if let Ok(mut outfile) = std::fs::File::create(&out_path) {
                            let _ = std::io::copy(&mut file, &mut outfile);
                        }
                    }
                }
            }
            let _ = std::fs::write(&version_file, current_version);
            eprintln!("[EMBED] Extensions extracted to {:?}", extensions_dir);
        }
    } else {
        eprintln!("[EMBED] Extensions up-to-date (v{})", current_version);
    }

    let _webview = WebViewBuilder::new()
        // Start with home URL
        .with_url(HOME_URL)
        // Disable devtools
        .with_devtools(false)
        // Persistent mode — keep cookies/sessions so logins survive restarts
        .with_incognito(false)
        // Allow clipboard
        .with_clipboard(true)
        // Allow zoom hotkeys
        .with_hotkeys_zoom(true)
        // Set a standard user agent for compatibility
        .with_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
        // Inject nav bar overlay on every page
        .with_initialization_script(NAV_BAR_JS)
        // Inject cosmetic ad blocker CSS
        .with_initialization_script(COSMETIC_BLOCKER_JS)
        // Inject YouTube ad skipper/blocker
        .with_initialization_script(YOUTUBE_AD_BLOCKER_JS)
        // Navigation handler — block ad URLs
        .with_navigation_handler(move |url: String| {
            if ad_blocker_nav.should_block(&url) {
                let count = blocked_count_nav.fetch_add(1, Ordering::Relaxed) + 1;
                eprintln!("[BLOCKED #{}] {}", count, url);
                false
            } else {
                true
            }
        })
        // Block popup/new window requests
        .with_new_window_req_handler(|url: String, _features| {
            eprintln!("[POPUP→DENY] {}", url);
            NewWindowResponse::Deny
        })
        // Update window title when page title changes
        .with_document_title_changed_handler({
            move |title| {
                eprintln!("[TITLE] {}", title);
            }
        })
        // === Windows-specific: Memory optimization + Extension support ===
        .with_additional_browser_args(
            [
                // Default wry flags (MUST re-add when using with_additional_browser_args)
                // ALL --disable-features merged into ONE flag (last one wins in Chromium!)
                "--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection,Prerender2",
                // === Memory Optimization (safe flags only) ===
                "--process-per-site",                 // Share process per site
                "--disable-gpu-memory-buffer-compositor-resources", // Reduce GPU memory
                "--js-flags=--max-old-space-size=256", // Limit V8 heap to 256MB (enough for extensions)
                "--disable-background-networking",    // No background network requests
                "--disable-client-side-phishing-detection", // Reduce memory
                "--no-pings",                         // Don't send hyperlink auditing pings
                // === Disable unnecessary features (but NOT extensions!) ===
                "--disable-sync",                     // No sync service
                "--disable-translate",                // No translation service
                "--disable-default-apps",             // No default apps
                "--disable-component-update",         // No component updates
                "--disable-background-timer-throttling", // Let our JS run efficiently
                "--no-proxy-server",                  // No proxy overhead
            ].join(" ")
        )
        .with_browser_extensions_enabled(true)
        .with_extensions_path(extensions_dir)
        .build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
