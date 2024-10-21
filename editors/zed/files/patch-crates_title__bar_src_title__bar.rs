--- crates/title_bar/src/title_bar.rs.orig	2024-10-16 18:48:23 UTC
+++ crates/title_bar/src/title_bar.rs
@@ -75,7 +75,7 @@ impl Render for TitleBar {
         let height = Self::height(cx);
         let supported_controls = cx.window_controls();
         let decorations = cx.window_decorations();
-        let titlebar_color = if cfg!(target_os = "linux") {
+        let titlebar_color = if cfg!(any(target_os = "linux", target_os = "freebsd")) {
             if cx.is_window_active() && !self.should_move {
                 cx.theme().colors().title_bar_background
             } else {
