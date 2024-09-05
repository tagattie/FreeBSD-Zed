--- crates/gpui/src/platform/blade/blade_renderer.rs.orig	2024-09-04 21:27:05 UTC
+++ crates/gpui/src/platform/blade/blade_renderer.rs
@@ -778,7 +778,7 @@ impl BladeRenderer {
     }
 
     /// Required to compile on macOS, but not currently supported.
-    #[cfg_attr(any(target_os = "linux", target_os = "windows"), allow(dead_code))]
+    #[cfg_attr(any(target_os = "linux", target_os = "windows", target_os = "freebsd"), allow(dead_code))]
     pub fn fps(&self) -> f32 {
         0.0
     }
