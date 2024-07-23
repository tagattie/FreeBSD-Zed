--- crates/gpui/src/platform/keystroke.rs.orig	2024-07-10 14:12:26 UTC
+++ crates/gpui/src/platform/keystroke.rs
@@ -190,7 +190,7 @@ impl std::fmt::Display for Keystroke {
             #[cfg(target_os = "macos")]
             f.write_char('⌘')?;
 
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             f.write_char('❖')?;
 
             #[cfg(target_os = "windows")]
