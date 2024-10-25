--- crates/gpui/src/platform/linux/xdg_desktop_portal.rs.orig	2024-10-24 17:42:16 UTC
+++ crates/gpui/src/platform/linux/xdg_desktop_portal.rs
@@ -162,7 +162,7 @@ impl WindowAppearance {
         }
     }
 
-    #[cfg_attr(target_os = "linux", allow(dead_code))]
+    #[cfg_attr(any(target_os = "linux", target_os = "freebsd"), allow(dead_code))]
     fn set_native(&mut self, cs: ColorScheme) {
         *self = Self::from_native(cs);
     }
