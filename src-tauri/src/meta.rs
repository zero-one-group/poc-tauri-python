pub const PKG_ARCH: &'static str = std::env::consts::ARCH;
pub const PKG_OS: &'static str = std::env::consts::OS;

pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const APP_TITLE: &'static str = "Tauri Python";
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub const MAIN_WINDOW: &'static str = "main";
pub const SETTING_WINDOW: &'static str = "app-setting";

// Informational metadata for the application
pub const FEEDBACK_URL: &'static str = "https://zero-one-group.com";

// Read value from envars, injected at compile time
// pub const API_BASE_URL: &'static str = env!("TAURI_API_BASE_URL");

// Disable webview native context menu.
// Optional, injected when webview loaded.
pub const JS_INIT_SCRIPT: &'static str = r#"
    (function() {
        document.addEventListener("contextmenu",
            (e) => { e.preventDefault(); return false; },
            { capture: true }
        );
    })();
"#;
