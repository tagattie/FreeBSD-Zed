--- crates/cli/src/main.rs.orig	2025-05-30 15:44:16 UTC
+++ crates/cli/src/main.rs
@@ -128,7 +128,7 @@ fn main() -> Result<()> {
 
 fn main() -> Result<()> {
     // Exit flatpak sandbox if needed
-    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
+    #[cfg(target_os = "linux")]
     {
         flatpak::try_restart_to_host();
         flatpak::ld_extra_libs();
@@ -152,7 +152,7 @@ fn main() -> Result<()> {
         paths::set_custom_data_dir(dir);
     }
 
-    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
+    #[cfg(target_os = "linux")]
     let args = flatpak::set_bin_if_no_escape(args);
 
     let app = Detect::detect(args.zed.as_deref()).context("Bundle detection")?;
@@ -359,7 +359,7 @@ fn anonymous_fd(path: &str) -> Option<fs::File> {
         let file = unsafe { fs::File::from_raw_fd(fd) };
         return Some(file);
     }
-    #[cfg(target_os = "macos")]
+    #[cfg(any(target_os = "macos", target_os = "freebsd"))]
     {
         use std::os::{
             fd::{self, FromRawFd},
@@ -377,7 +377,7 @@ fn anonymous_fd(path: &str) -> Option<fs::File> {
         let file = unsafe { fs::File::from_raw_fd(fd) };
         return Some(file);
     }
-    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
+    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
     {
         _ = path;
         // not implemented for bsd, windows. Could be, but isn't yet
@@ -522,7 +522,7 @@ mod linux {
     }
 }
 
-#[cfg(any(target_os = "linux", target_os = "freebsd"))]
+#[cfg(target_os = "linux")]
 mod flatpak {
     use std::ffi::OsString;
     use std::path::PathBuf;
