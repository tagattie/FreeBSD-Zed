--- crates/editor/src/element.rs.orig	2024-10-24 17:42:16 UTC
+++ crates/editor/src/element.rs
@@ -645,12 +645,12 @@ impl EditorElement {
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
             if EditorSettings::get_global(cx).middle_click_paste {
                 if let Some(text) = cx.read_from_primary().and_then(|item| item.text()) {
                     let point_for_position =
