--- crates/editor/src/element.rs.orig	2024-10-02 15:03:39 UTC
+++ crates/editor/src/element.rs
@@ -651,7 +651,7 @@ impl EditorElement {
             cx.stop_propagation();
         } else if end_selection && pending_nonempty_selections {
             cx.stop_propagation();
-        } else if cfg!(target_os = "linux")
+        } else if cfg!(any(target_os = "linux", target_os = "freebsd"))
             && event.button == MouseButton::Middle
             && (!text_hitbox.is_hovered(cx) || editor.read_only(cx))
         {
