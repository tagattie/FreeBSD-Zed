--- crates/gpui/src/style.rs.orig	2024-07-10 14:12:26 UTC
+++ crates/gpui/src/style.rs
@@ -210,7 +210,7 @@ impl Default for TextStyle {
         TextStyle {
             color: black(),
             // todo(linux) make this configurable or choose better default
-            font_family: if cfg!(target_os = "linux") {
+            font_family: if cfg!(any(target_os = "linux", target_os = "freebsd")) {
                 "FreeMono".into()
             } else {
                 "Helvetica".into()
