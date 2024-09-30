--- crates/gpui/src/window.rs.orig	2024-09-27 20:04:41 UTC
+++ crates/gpui/src/window.rs
@@ -1243,7 +1243,7 @@ impl<'a> WindowContext<'a> {
     /// that currently owns the mouse cursor.
     /// On mac, this is equivalent to `is_window_active`.
     pub fn is_window_hovered(&self) -> bool {
-        if cfg!(target_os = "linux") {
+        if cfg!(any(target_os = "linux", target_os = "freebsd")) {
             self.window.hovered.get()
         } else {
             self.is_window_active()
