--- crates/ui/src/styles/platform.rs.orig	2024-05-01 08:24:10 UTC
+++ crates/ui/src/styles/platform.rs
@@ -14,7 +14,7 @@ impl PlatformStyle {
 impl PlatformStyle {
     /// Returns the [`PlatformStyle`] for the current platform.
     pub const fn platform() -> Self {
-        if cfg!(target_os = "linux") {
+        if cfg!(any(target_os = "linux", target_os = "freebsd")) {
             Self::Linux
         } else if cfg!(target_os = "windows") {
             Self::Windows
