--- crates/paths/src/paths.rs.orig	2024-10-24 17:42:16 UTC
+++ crates/paths/src/paths.rs
@@ -20,7 +20,7 @@ pub fn config_dir() -> &'static PathBuf {
                 .join("Zed");
         }
 
-        if cfg!(target_os = "linux") {
+        if cfg!(any(target_os = "linux", target_os = "freebsd")) {
             return if let Ok(flatpak_xdg_config) = std::env::var("FLATPAK_XDG_CONFIG_HOME") {
                 flatpak_xdg_config.into()
             } else {
@@ -41,7 +41,7 @@ pub fn support_dir() -> &'static PathBuf {
             return home_dir().join("Library/Application Support/Zed");
         }
 
-        if cfg!(target_os = "linux") {
+        if cfg!(any(target_os = "linux", target_os = "freebsd")) {
             return if let Ok(flatpak_xdg_data) = std::env::var("FLATPAK_XDG_DATA_HOME") {
                 flatpak_xdg_data.into()
             } else {
@@ -76,7 +76,7 @@ pub fn temp_dir() -> &'static PathBuf {
                 .join("Zed");
         }
 
-        if cfg!(target_os = "linux") {
+        if cfg!(any(target_os = "linux", target_os = "freebsd")) {
             return if let Ok(flatpak_xdg_cache) = std::env::var("FLATPAK_XDG_CACHE_HOME") {
                 flatpak_xdg_cache.into()
             } else {
