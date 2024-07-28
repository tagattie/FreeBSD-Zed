--- crates/markdown/src/markdown.rs.orig	2024-07-24 16:08:14 UTC
+++ crates/markdown/src/markdown.rs
@@ -453,7 +453,7 @@ impl MarkdownElement {
                 } else {
                     if markdown.selection.pending {
                         markdown.selection.pending = false;
-                        #[cfg(target_os = "linux")]
+                        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                         {
                             let text = rendered_text
                                 .text_for_range(markdown.selection.start..markdown.selection.end);
