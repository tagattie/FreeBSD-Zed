--- crates/editor/src/editor.rs.orig	2024-10-16 18:48:23 UTC
+++ crates/editor/src/editor.rs
@@ -2470,7 +2470,7 @@ impl Editor {
         cx.invalidate_character_coordinates();
 
         // Copy selections to primary selection buffer
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         if local {
             let selections = self.selections.all::<usize>(cx);
             let buffer_handle = self.buffer.read(cx).read(cx);
