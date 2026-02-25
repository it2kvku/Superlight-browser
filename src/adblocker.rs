use std::collections::HashSet;

/// Simple, fast ad blocker using domain-based blocking.
/// Rules are compiled into the binary — no external files needed.
pub struct AdBlocker {
    blocked_domains: HashSet<&'static str>,
    blocked_patterns: Vec<&'static str>,
}

impl AdBlocker {
    pub fn new() -> Self {
        let blocked_domains: HashSet<&'static str> = [
            // === Major Ad Networks ===
            "doubleclick.net",
            "googlesyndication.com",
            "googleadservices.com",
            "google-analytics.com",
            "googletagmanager.com",
            "googletagservices.com",
            "pagead2.googlesyndication.com",
            "adservice.google.com",
            "ads.google.com",
            "ade.googlesyndication.com",
            "adclick.g.doubleclick.net",
            "ad.doubleclick.net",
            "static.doubleclick.net",
            "m.doubleclick.net",
            "mediavisor.doubleclick.net",
            // === Facebook / Meta Ads ===
            "an.facebook.com",
            "pixel.facebook.com",
            "ads.facebook.com",
            // === Amazon Ads ===
            "aax.amazon-adsystem.com",
            "z-na.amazon-adsystem.com",
            "s.amazon-adsystem.com",
            "c.amazon-adsystem.com",
            "amazon-adsystem.com",
            // === Microsoft / Bing Ads ===
            "ads.bing.com",
            "bat.bing.com",
            // === Twitter / X Ads ===
            "ads-api.twitter.com",
            "ads.twitter.com",
            "analytics.twitter.com",
            // === Common Ad Networks ===
            "adnxs.com",
            "adsrvr.org",
            "adform.net",
            "adcolony.com",
            "admob.com",
            "advertising.com",
            "contextweb.com",
            "criteo.com",
            "criteo.net",
            "crwdcntrl.net",
            "demdex.net",
            "exoclick.com",
            "exponential.com",
            "eyereturn.com",
            "flashtalking.com",
            "freewheel.com",
            "inmobi.com",
            "innovid.com",
            "ipredictive.com",
            "liadm.com",
            "lkqd.net",
            "mathtag.com",
            "media.net",
            "mediaplex.com",
            "moatads.com",
            "mookie1.com",
            "netmng.com",
            "openx.net",
            "outbrain.com",
            "popads.net",
            "popcash.net",
            "propellerads.com",
            "pubmatic.com",
            "revjet.com",
            "rfihub.com",
            "richrelevance.com",
            "rlcdn.com",
            "rubiconproject.com",
            "servebom.com",
            "sharethis.com",
            "smaato.net",
            "smartadserver.com",
            "spotxchange.com",
            "taboola.com",
            "tapad.com",
            "tidaltv.com",
            "trafficjunky.com",
            "tribalfusion.com",
            "turn.com",
            "undertone.com",
            "unityads.unity3d.com",
            "yieldmo.com",
            "yimg.com",
            "zedo.com",
            // === Trackers ===
            "scorecardresearch.com",
            "quantserve.com",
            "bluekai.com",
            "exelator.com",
            "agkn.com",
            "adsymptotic.com",
            "segment.com",
            "segment.io",
            "amplitude.com",
            "mixpanel.com",
            "hotjar.com",
            "mouseflow.com",
            "fullstory.com",
            "luckyorange.com",
            "crazyegg.com",
            "clicktale.net",
            "optimizely.com",
            "omtrdc.net",
            "2o7.net",
            "omniture.com",
            "hit.gemius.pl",
            "chartbeat.com",
            "chartbeat.net",
            "newrelic.com",
            "nr-data.net",
            "branch.io",
            "app.link",
            "appsflyer.com",
            "adjust.com",
            "kochava.com",
            "mxpnl.com",
            "braze.com",
            "appboy.com",
            // === Malware / Scam ===
            "malware-check.disconnect.me",
            // === Video Ads ===
            "imasdk.googleapis.com",
            "static.ads-twitter.com",
            "syndication.twitter.com",
            "vid.springserve.com",
            // === YouTube Ad Servers ===
            "ads.youtube.com",
            "youtube.cleverads.vn",
            "www.youtube-nocookie.com",
            "s.youtube.com",
            "video-ad-stats.googlesyndication.com",
            "pagead2.googlesyndication.com",
            "pubads.g.doubleclick.net",
            "securepubads.g.doubleclick.net",
            "www.googleadservices.com",
            "ytimg.l.google.com",
            "ad.youtube.com",
            "yt3.ggpht.com",
            // === Pop-ups / Overlays ===
            "cdn.taboola.com",
            "cdn.outbrain.com",
            "widgets.outbrain.com",
            "static.taboola.com",
            // === Cookie Consent Trackers ===
            "consent.cookiebot.com",
            "cdn.cookielaw.org",
            // === Additional High-Volume Ad Domains ===
            "securepubads.g.doubleclick.net",
            "tpc.googlesyndication.com",
            "pagead2.googlesyndication.com",
            "ad.atdmt.com",
            "adserver.yahoo.com",
            "yieldmanager.com",
            "serving-sys.com",
            "eyeblaster.com",
            "adap.tv",
            "revsci.net",
            "nexac.com",
            "adadvisor.net",
            "adbureau.net",
            "advertising.com",
            "ru4.com",
            "adbrite.com",
            "adsonar.com",
            "atwola.com",
            "crowdscience.com",
            "collective-media.net",
            "specificmedia.com",
            "trafficmp.com",
            "adtech.de",
            "bannersxchange.com",
            // === More Trackers ===
            "eum-appdynamics.com",
            "d.agkn.com",
            "bidswitch.net",
            "casalemedia.com",
            "lijit.com",
            "addthis.com",
            "adobedtm.com",
            "bkrtx.com",
            "bttrack.com",
            "everesttech.net",
            "mxptint.net",
            "narrative.io",
            "onetrust.com",
            "pardot.com",
            "pippio.com",
            "px.ads.linkedin.com",
            "scdn.co",
            "sitescout.com",
            "snapchat.com",
            "sspx.co",
            "t.co",
            "teads.tv",
            "theadex.com",
            "tremorhub.com",
            "triton.com",
            "zemanta.com",
        ]
        .into_iter()
        .collect();

        let blocked_patterns = vec![
            "/ads/",
            "/ad/",
            "/adserver",
            "/adframe",
            "/adcontent",
            "/adview",
            "/adclick",
            "/adsense",
            "/adsbygoogle",
            "/pagead/",
            "/sponsor",
            "/banner/",
            "/banners/",
            "/pop_ads",
            "/popup_ad",
            "/tracking/",
            "/tracker/",
            "/pixel/",
            "/beacon/",
            "/collect?",
            "doubleclick.net/",
            "/prebid",
            "/gpt/pubads",
            "_ad_",
            "-ad-",
            ".ad.",
            "/analytics.js",
            "/gtag/",
            "/gtm.js",
            // === YouTube Ad Patterns ===
            "/pagead/",
            "/ptracking",
            "/api/stats/ads",
            "/api/stats/atr",
            "/get_video_info?",
            "googlevideo.com/videogoodput",
            "googlevideo.com/initplayback",
            "&ad_type=",
            "&adurl=",
            "youtube.com/api/stats/qoe?adformat",
            "youtube.com/pagead/",
            "youtube.com/ptracking",
            "youtube.com/get_midroll",
            "/generate_204",
        ];

        Self {
            blocked_domains,
            blocked_patterns,
        }
    }

    /// Check if a URL should be blocked
    pub fn should_block(&self, url: &str) -> bool {
        let url_lower = url.to_lowercase();

        // Extract domain from URL
        if let Some(domain) = extract_domain(&url_lower) {
            // Check exact domain match
            if self.blocked_domains.contains(domain.as_str()) {
                return true;
            }
            // Check if any parent domain is blocked
            // e.g., "sub.doubleclick.net" should be blocked because "doubleclick.net" is
            let parts: Vec<&str> = domain.split('.').collect();
            for i in 0..parts.len().saturating_sub(1) {
                let parent = parts[i..].join(".");
                if self.blocked_domains.contains(parent.as_str()) {
                    return true;
                }
            }
        }

        // Check URL patterns
        for pattern in &self.blocked_patterns {
            if url_lower.contains(pattern) {
                return true;
            }
        }

        false
    }
}

/// Extract the domain from a URL string
fn extract_domain(url: &str) -> Option<String> {
    // Remove protocol
    let without_protocol = if let Some(pos) = url.find("://") {
        &url[pos + 3..]
    } else {
        url
    };

    // Get domain part (before first / or ? or #)
    let domain = without_protocol
        .split('/')
        .next()?
        .split('?')
        .next()?
        .split('#')
        .next()?;

    // Remove port if present
    let domain = if let Some(pos) = domain.rfind(':') {
        // Check it's not part of IPv6
        if domain.starts_with('[') {
            domain
        } else {
            &domain[..pos]
        }
    } else {
        domain
    };

    if domain.is_empty() {
        None
    } else {
        Some(domain.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocks_known_ad_domains() {
        let blocker = AdBlocker::new();
        assert!(blocker.should_block("https://doubleclick.net/some-ad"));
        assert!(blocker.should_block("https://ad.doubleclick.net/something"));
        assert!(blocker.should_block("https://googlesyndication.com/pagead"));
        assert!(blocker.should_block("https://ads.facebook.com/track"));
    }

    #[test]
    fn test_allows_normal_urls() {
        let blocker = AdBlocker::new();
        assert!(!blocker.should_block("https://www.google.com"));
        assert!(!blocker.should_block("https://github.com"));
        assert!(!blocker.should_block("https://stackoverflow.com/questions"));
        assert!(!blocker.should_block("https://www.youtube.com/watch?v=abc"));
    }

    #[test]
    fn test_blocks_url_patterns() {
        let blocker = AdBlocker::new();
        assert!(blocker.should_block("https://example.com/ads/banner.jpg"));
        assert!(blocker.should_block("https://example.com/adserver/get"));
        assert!(blocker.should_block("https://example.com/tracking/pixel"));
    }

    #[test]
    fn test_blocks_subdomains() {
        let blocker = AdBlocker::new();
        assert!(blocker.should_block("https://sub.criteo.com/something"));
        assert!(blocker.should_block("https://cdn.taboola.com/script.js"));
    }
}
