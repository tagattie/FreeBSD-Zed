--- crates/paths/src/paths.rs.orig	2024-07-11 12:09:42 UTC
+++ crates/paths/src/paths.rs
@@ -15,7 +15,7 @@ pub fn config_dir() -> &'static PathBuf {
                 .join("Zed");
         }
 
-        if cfg!(target_os = "linux") {
+        if cfg!(any(target_os = "linux", target_os = "freebsd")) {
             return if let Ok(flatpak_xdg_config) = std::env::var("FLATPAK_XDG_CONFIG_HOME") {
                 flatpak_xdg_config.into()
             } else {
@@ -36,7 +36,7 @@ pub fn support_dir() -> &'static PathBuf {
             return home_dir().join("Library/Application Support/Zed");
         }
 
-        if cfg!(target_os = "linux") {
+        if cfg!(any(target_os = "linux", target_os = "freebsd")) {
             return if let Ok(flatpak_xdg_data) = std::env::var("FLATPAK_XDG_DATA_HOME") {
                 flatpak_xdg_data.into()
             } else {
@@ -65,7 +65,7 @@ pub fn temp_dir() -> &'static PathBuf {
                 .join("Zed");
         }
 
-        if cfg!(target_os = "linux") {
+        if cfg!(any(target_os = "linux", target_os = "freebsd")) {
             return if let Ok(flatpak_xdg_cache) = std::env::var("FLATPAK_XDG_CACHE_HOME") {
                 flatpak_xdg_cache.into()
             } else {
