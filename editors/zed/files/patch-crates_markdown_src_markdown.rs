--- crates/markdown/src/markdown.rs.orig	2024-08-27 11:06:16 UTC
+++ crates/markdown/src/markdown.rs
@@ -476,7 +476,7 @@ impl MarkdownElement {
                 } else {
                     if markdown.selection.pending {
                         markdown.selection.pending = false;
-                        #[cfg(target_os = "linux")]
+                        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                         {
                             let text = rendered_text
                                 .text_for_range(markdown.selection.start..markdown.selection.end);
