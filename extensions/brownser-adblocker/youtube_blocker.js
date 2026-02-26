// Brownser YouTube Ad Blocker — Content Script
// Runs at document_start on youtube.com
(function() {
    'use strict';

    // === 1. INTERCEPT AD REQUESTS VIA XHR/FETCH ===
    // Override XMLHttpRequest.open to block ad-related API calls
    const origXHROpen = XMLHttpRequest.prototype.open;
    XMLHttpRequest.prototype.open = function(method, url) {
        const urlStr = String(url || '');
        if (urlStr.includes('/pagead/') ||
            urlStr.includes('/ptracking') ||
            urlStr.includes('/api/stats/ads') ||
            urlStr.includes('/api/stats/atr') ||
            urlStr.includes('get_midroll') ||
            urlStr.includes('/generate_204') ||
            urlStr.includes('/error_204') ||
            urlStr.includes('&ad_type=') ||
            urlStr.includes('&adurl=') ||
            urlStr.includes('adformat=') ||
            urlStr.includes('ad_logging_flag') ||
            urlStr.includes('googlesyndication.com') ||
            urlStr.includes('doubleclick.net') ||
            urlStr.includes('googleadservices.com') ||
            urlStr.includes('imasdk.googleapis.com') ||
            urlStr.includes('video-ad-stats') ||
            urlStr.includes('videogoodput') ||
            urlStr.includes('initplayback')) {
            // Redirect to a dead URL
            return origXHROpen.apply(this, [method, 'data:text/plain,blocked', false]);
        }
        return origXHROpen.apply(this, arguments);
    };

    // Override fetch to block ad requests
    const origFetch = window.fetch;
    window.fetch = function(input, init) {
        const url = (typeof input === 'string') ? input :
                    (input instanceof Request) ? input.url : String(input);
        if (url.includes('/pagead/') ||
            url.includes('/ptracking') ||
            url.includes('/api/stats/ads') ||
            url.includes('/api/stats/atr') ||
            url.includes('get_midroll') ||
            url.includes('&ad_type=') ||
            url.includes('&adurl=') ||
            url.includes('adformat=') ||
            url.includes('googlesyndication.com') ||
            url.includes('doubleclick.net') ||
            url.includes('googleadservices.com') ||
            url.includes('imasdk.googleapis.com')) {
            return Promise.resolve(new Response('', { status: 200, statusText: 'Blocked' }));
        }
        return origFetch.apply(this, arguments);
    };

    // === 2. CSS: HIDE ALL AD UI ELEMENTS ===
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
        .ytp-ad-action-interstitial,
        .ytp-ad-action-interstitial-background-container,
        .ytp-ad-action-interstitial-slot,
        .ytp-ad-progress-list,
        .ytp-ad-player-overlay,
        .ytp-ad-player-overlay-layout,
        .ytp-ad-message-container,

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

        /* Companion ads (sidebar) */
        ytd-companion-slot-renderer,
        .ytd-companion-slot-renderer,
        #companion,
        ytd-merch-shelf-renderer,

        /* Premium upsell */
        ytd-mealbar-promo-renderer,
        tp-yt-paper-dialog:has(yt-mealbar-promo-renderer),
        tp-yt-paper-dialog:has(ytd-enforcement-message-view-model),
        ytmusic-mealbar-promo-renderer,

        /* Sponsored labels */
        .badge-style-type-ad,
        .sparkles-light-cta,
        .google-revocation-link-element,

        /* "Ad" text on video player */
        .ytp-ad-text,
        .ytp-ad-preview-text,
        .ytp-ad-simple-ad-badge {
            display: none !important;
            visibility: hidden !important;
            height: 0 !important;
            max-height: 0 !important;
            overflow: hidden !important;
        }

        /* Make video visible even during ad state */
        .ad-showing .html5-video-container { display: block !important; }
        .ad-showing video { visibility: visible !important; }
    `;
    document.documentElement.appendChild(style);

    // === 3. AUTO-SKIP VIDEO ADS ===
    function trySkipAd() {
        // Click skip button (multiple selectors for different YT versions)
        const skipSelectors = [
            '.ytp-ad-skip-button',
            '.ytp-ad-skip-button-modern',
            '.ytp-skip-ad-button',
            'button.ytp-ad-skip-button-modern',
            '.ytp-ad-skip-button-container button',
            '[id^="skip-button"]',
            '.ytp-ad-skip-button-text',
            'button[id^="skip-button"]',
            '.ytp-ad-overlay-close-button',
        ];
        for (const sel of skipSelectors) {
            const btns = document.querySelectorAll(sel);
            for (const btn of btns) {
                if (btn && btn.offsetParent !== null) {
                    btn.click();
                    return true;
                }
            }
        }

        // Fast-forward unskippable ads
        const player = document.querySelector('.ad-showing video');
        if (player && player.duration && player.duration > 0 && isFinite(player.duration)) {
            player.currentTime = player.duration - 0.1;
            player.muted = true;
            try { 
                player.playbackRate = 16; 
            } catch(e) {}
            return true;
        }

        return false;
    }

    let skipTimer = null;
    function startSkipping() {
        if (skipTimer) return;
        trySkipAd();
        let attempts = 0;
        skipTimer = setInterval(() => {
            if (!document.querySelector('.ad-showing')) {
                clearInterval(skipTimer);
                skipTimer = null;
                return;
            }
            trySkipAd();
            attempts++;
            if (attempts > 200) {
                clearInterval(skipTimer);
                skipTimer = null;
            }
        }, 50); // Check every 50ms for fast response
    }

    // === 4. MUTATION OBSERVER — detect ads in real-time ===
    const observer = new MutationObserver(function(mutations) {
        for (const mutation of mutations) {
            // Class changes on player (ad-showing added)
            if (mutation.type === 'attributes' && mutation.attributeName === 'class') {
                if (mutation.target.classList && mutation.target.classList.contains('ad-showing')) {
                    startSkipping();
                    return;
                }
            }
            // New ad nodes added
            for (const node of mutation.addedNodes) {
                if (node.nodeType !== 1) continue;
                const tag = node.tagName ? node.tagName.toLowerCase() : '';
                // Remove ad slot renderers immediately
                if (tag === 'ytd-ad-slot-renderer' ||
                    tag === 'ytd-in-feed-ad-layout-renderer' ||
                    tag === 'ytd-banner-promo-renderer' ||
                    tag === 'ytd-promoted-sparkles-web-renderer' ||
                    tag === 'ytd-display-ad-renderer' ||
                    tag === 'ytd-compact-promoted-item-renderer') {
                    node.remove();
                    continue;
                }
                // Check for ad-showing class
                if (node.classList && (
                    node.classList.contains('ytp-ad-module') ||
                    node.classList.contains('ytp-ad-overlay-container') ||
                    node.classList.contains('ad-showing') ||
                    node.classList.contains('ytp-ad-player-overlay'))) {
                    startSkipping();
                }
            }
        }
    });

    function startObserving() {
        observer.observe(document.documentElement, {
            childList: true,
            subtree: true,
            attributes: true,
            attributeFilter: ['class']
        });
    }

    if (document.body) {
        startObserving();
    } else {
        document.addEventListener('DOMContentLoaded', startObserving);
    }

    // === 5. PERIODIC CLEANUP (fallback) ===
    setInterval(() => {
        // Remove feed ads
        document.querySelectorAll(
            'ytd-ad-slot-renderer, ytd-in-feed-ad-layout-renderer, ' +
            'ytd-banner-promo-renderer, ytd-promoted-sparkles-web-renderer, ' +
            'ytd-display-ad-renderer, ytd-compact-promoted-item-renderer, ' +
            'ytd-player-legacy-desktop-watch-ads-renderer'
        ).forEach(el => el.remove());

        // Check for active video ads
        if (document.querySelector('.ad-showing')) {
            startSkipping();
        }
    }, 500);

    // Initial check
    if (document.querySelector('.ad-showing')) {
        startSkipping();
    }

    console.log('[Brownser] YouTube ad blocker extension active');
})();
