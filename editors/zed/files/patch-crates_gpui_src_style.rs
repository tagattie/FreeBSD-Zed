--- crates/gpui/src/style.rs.orig	2024-08-27 11:06:16 UTC
+++ crates/gpui/src/style.rs
@@ -328,7 +328,7 @@ impl Default for TextStyle {
         TextStyle {
             color: black(),
             // todo(linux) make this configurable or choose better default
-            font_family: if cfg!(target_os = "linux") {
+            font_family: if cfg!(any(target_os = "linux", target_os = "freebsd")) {
                 "FreeMono".into()
             } else if cfg!(target_os = "windows") {
                 "Segoe UI".into()
