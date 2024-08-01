--- crates/gpui/src/platform/keystroke.rs.orig	2024-07-31 16:17:45 UTC
+++ crates/gpui/src/platform/keystroke.rs
@@ -181,7 +181,7 @@ impl std::fmt::Display for Keystroke {
             #[cfg(target_os = "macos")]
             f.write_char('⌘')?;
 
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             f.write_char('❖')?;
 
             #[cfg(target_os = "windows")]
