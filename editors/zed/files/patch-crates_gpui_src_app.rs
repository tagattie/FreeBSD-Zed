--- crates/gpui/src/app.rs.orig	2024-08-27 11:06:16 UTC
+++ crates/gpui/src/app.rs
@@ -565,7 +565,7 @@ impl AppContext {
 
     /// Writes data to the primary selection buffer.
     /// Only available on Linux.
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     pub fn write_to_primary(&self, item: ClipboardItem) {
         self.platform.write_to_primary(item)
     }
@@ -577,7 +577,7 @@ impl AppContext {
 
     /// Reads data from the primary selection buffer.
     /// Only available on Linux.
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     pub fn read_from_primary(&self) -> Option<ClipboardItem> {
         self.platform.read_from_primary()
     }
