--- crates/gpui/src/keymap/context.rs.orig	2024-07-11 12:04:01 UTC
+++ crates/gpui/src/keymap/context.rs
@@ -30,11 +30,11 @@ impl KeyContext {
         let mut context = Self::default();
         #[cfg(target_os = "macos")]
         context.set("os", "macos");
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         context.set("os", "linux");
         #[cfg(target_os = "windows")]
         context.set("os", "windows");
-        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
+        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "freebsd", target_os = "windows")))]
         context.set("os", "unknown");
         context
     }
