--- crates/gpui/src/platform/keystroke.rs.orig	2024-09-04 14:30:20 UTC
+++ crates/gpui/src/platform/keystroke.rs
@@ -182,7 +182,7 @@ impl std::fmt::Display for Keystroke {
             #[cfg(target_os = "macos")]
             f.write_char('⌘')?;
 
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             f.write_char('❖')?;
 
             #[cfg(target_os = "windows")]
