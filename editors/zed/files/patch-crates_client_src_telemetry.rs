--- crates/client/src/telemetry.rs.orig	2024-10-16 18:48:23 UTC
+++ crates/client/src/telemetry.rs
@@ -106,6 +106,10 @@ pub fn os_name() -> String {
     {
         format!("Linux {}", gpui::guess_compositor())
     }
+    #[cfg(target_os = "freebsd")]
+    {
+        format!("FreeBSD {}", gpui::guess_compositor())
+    }
 
     #[cfg(target_os = "windows")]
     {
@@ -131,7 +135,7 @@ pub fn os_version() -> String {
             .to_string()
         }
     }
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     {
         use std::path::Path;
 
