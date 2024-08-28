--- crates/title_bar/src/title_bar.rs.orig	2024-08-27 11:06:16 UTC
+++ crates/title_bar/src/title_bar.rs
@@ -74,7 +74,7 @@ impl Render for TitleBar {
         let height = Self::height(cx);
         let supported_controls = cx.window_controls();
         let decorations = cx.window_decorations();
-        let titlebar_color = if cfg!(target_os = "linux") {
+        let titlebar_color = if cfg!(any(target_os = "linux", target_os = "freebsd")) {
             if cx.is_window_active() {
                 cx.theme().colors().title_bar_background
             } else {
