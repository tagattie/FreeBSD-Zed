--- crates/editor/src/element.rs.orig	2024-08-27 11:06:16 UTC
+++ crates/editor/src/element.rs
@@ -643,12 +643,12 @@ impl EditorElement {
             cx.stop_propagation();
         } else if end_selection && pending_nonempty_selections {
             cx.stop_propagation();
-        } else if cfg!(target_os = "linux") && event.button == MouseButton::Middle {
+        } else if cfg!(any(target_os = "linux", target_os = "freebsd")) && event.button == MouseButton::Middle {
             if !text_hitbox.is_hovered(cx) || editor.read_only(cx) {
                 return;
             }
 
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             if let Some(text) = cx.read_from_primary().and_then(|item| item.text()) {
                 let point_for_position =
                     position_map.point_for_position(text_hitbox.bounds, event.position);
