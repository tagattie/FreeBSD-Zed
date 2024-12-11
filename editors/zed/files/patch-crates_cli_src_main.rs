--- crates/cli/src/main.rs.orig	2024-12-04 17:01:03 UTC
+++ crates/cli/src/main.rs
@@ -91,7 +91,7 @@ fn main() -> Result<()> {
 
 fn main() -> Result<()> {
     // Exit flatpak sandbox if needed
-    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
+    #[cfg(target_os = "linux")]
     {
         flatpak::try_restart_to_host();
         flatpak::ld_extra_libs();
@@ -109,7 +109,7 @@ fn main() -> Result<()> {
     }
     let args = Args::parse();
 
-    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
+    #[cfg(target_os = "linux")]
     let args = flatpak::set_bin_if_no_escape(args);
 
     let app = Detect::detect(args.zed.as_deref()).context("Bundle detection")?;
@@ -347,7 +347,7 @@ mod linux {
     }
 }
 
-#[cfg(any(target_os = "linux", target_os = "freebsd"))]
+#[cfg(target_os = "linux")]
 mod flatpak {
     use std::ffi::OsString;
     use std::path::PathBuf;
