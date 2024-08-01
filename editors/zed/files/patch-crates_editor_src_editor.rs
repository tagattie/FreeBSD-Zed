--- crates/editor/src/editor.rs.orig	2024-07-31 16:17:45 UTC
+++ crates/editor/src/editor.rs
@@ -2285,7 +2285,7 @@ impl Editor {
         cx: &mut ViewContext<Self>,
     ) {
         // Copy selections to primary selection buffer
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         if local {
             let selections = self.selections.all::<usize>(cx);
             let buffer_handle = self.buffer.read(cx).read(cx);
