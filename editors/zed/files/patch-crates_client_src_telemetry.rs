--- crates/client/src/telemetry.rs.orig	2025-05-14 14:15:56 UTC
+++ crates/client/src/telemetry.rs
@@ -96,9 +96,13 @@ pub fn os_name() -> String {
     {
         "macOS".to_string()
     }
-    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
+    #[cfg(target_os = "linux")]
     {
         format!("Linux {}", gpui::guess_compositor())
+    }
+    #[cfg(target_os = "freebsd")]
+    {
+        format!("FreeBSD {}", gpui::guess_compositor())
     }
 
     #[cfg(target_os = "windows")]
