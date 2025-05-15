--- crates/cli/src/main.rs.orig	2025-05-14 14:15:56 UTC
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
@@ -525,7 +525,7 @@ mod linux {
     }
 }
 
-#[cfg(any(target_os = "linux", target_os = "freebsd"))]
+#[cfg(target_os = "linux")]
 mod flatpak {
     use std::ffi::OsString;
     use std::path::PathBuf;
