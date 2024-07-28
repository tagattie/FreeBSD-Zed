--- crates/editor/src/editor.rs.orig	2024-07-24 16:08:14 UTC
+++ crates/editor/src/editor.rs
@@ -2264,7 +2264,7 @@ impl Editor {
         cx: &mut ViewContext<Self>,
     ) {
         // Copy selections to primary selection buffer
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         if local {
             let selections = self.selections.all::<usize>(cx);
             let buffer_handle = self.buffer.read(cx).read(cx);
