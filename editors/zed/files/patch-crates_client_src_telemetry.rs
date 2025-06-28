--- crates/client/src/telemetry.rs.orig	2025-06-27 16:44:45 UTC
+++ crates/client/src/telemetry.rs
@@ -83,10 +83,14 @@ pub fn os_name() -> String {
     {
         "macOS".to_string()
     }
-    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
+    #[cfg(target_os = "linux")]
     {
         format!("Linux {}", gpui::guess_compositor())
     }
+    #[cfg(target_os = "freebsd")]
+    {
+        format!("FreeBSD {}", gpui::guess_compositor())
+    }
 
     #[cfg(target_os = "windows")]
     {
@@ -119,6 +123,8 @@ pub fn os_version() -> String {
         let content = if let Ok(file) = std::fs::read_to_string(&Path::new("/etc/os-release")) {
             file
         } else if let Ok(file) = std::fs::read_to_string(&Path::new("/usr/lib/os-release")) {
+            file
+        } else if let Ok(file) = std::fs::read_to_string(&Path::new("/var/run/os-release")) {
             file
         } else {
             log::error!("Failed to load /etc/os-release, /usr/lib/os-release");
