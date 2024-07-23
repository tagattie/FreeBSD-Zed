--- crates/client/src/telemetry.rs.orig	2024-07-11 11:53:24 UTC
+++ crates/client/src/telemetry.rs
@@ -104,6 +104,10 @@ pub fn os_name() -> String {
     {
         format!("Linux {}", gpui::guess_compositor())
     }
+    #[cfg(target_os = "freebsd")]
+    {
+        format!("FreeBSD {}", gpui::guess_compositor())
+    }
 
     #[cfg(target_os = "windows")]
     {
@@ -129,7 +133,7 @@ pub fn os_version() -> String {
             .to_string()
         }
     }
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     {
         use std::path::Path;
 
