--- crates/gpui/src/platform/blade/blade_renderer.rs.orig	2025-01-13 22:44:47 UTC
+++ crates/gpui/src/platform/blade/blade_renderer.rs
@@ -396,7 +396,7 @@ impl BladeRenderer {
     /// Like `update_drawable_size` but skips the check that the size has changed. This is useful in
     /// cases like restoring a window from minimization where the size is the same but the
     /// renderer's swap chain needs to be recreated.
-    #[cfg_attr(any(target_os = "macos", target_os = "linux"), allow(dead_code))]
+    #[cfg_attr(any(target_os = "macos", target_os = "linux", target_os = "freebsd"), allow(dead_code))]
     pub fn update_drawable_size_even_if_unchanged(&mut self, size: Size<DevicePixels>) {
         self.update_drawable_size_impl(size, true);
     }
