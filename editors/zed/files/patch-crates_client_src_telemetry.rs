--- crates/client/src/telemetry.rs.orig	2024-12-18 16:41:33 UTC
+++ crates/client/src/telemetry.rs
@@ -99,9 +99,13 @@ pub fn os_name() -> String {
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
