--- crates/client/src/telemetry.rs.orig	2024-09-27 20:04:41 UTC
+++ crates/client/src/telemetry.rs
@@ -105,6 +105,10 @@ pub fn os_name() -> String {
     {
         format!("Linux {}", gpui::guess_compositor())
     }
+    #[cfg(target_os = "freebsd")]
+    {
+        format!("FreeBSD {}", gpui::guess_compositor())
+    }
 
     #[cfg(target_os = "windows")]
     {
@@ -130,7 +134,7 @@ pub fn os_version() -> String {
             .to_string()
         }
     }
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     {
         use std::path::Path;
 
