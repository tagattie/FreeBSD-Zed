--- crates/editor/src/element.rs.orig	2024-07-24 16:08:14 UTC
+++ crates/editor/src/element.rs
@@ -597,12 +597,12 @@ impl EditorElement {
             cx.stop_propagation();
         } else if end_selection && pending_nonempty_selections {
             cx.stop_propagation();
-        } else if cfg!(target_os = "linux") && event.button == MouseButton::Middle {
+        } else if (cfg!(target_os = "linux") || cfg!(target_os = "freebsd")) && event.button == MouseButton::Middle {
             if !text_hitbox.is_hovered(cx) || editor.read_only(cx) {
                 return;
             }
 
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             if let Some(item) = cx.read_from_primary() {
                 let point_for_position =
                     position_map.point_for_position(text_hitbox.bounds, event.position);
