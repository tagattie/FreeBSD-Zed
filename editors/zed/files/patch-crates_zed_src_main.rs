--- crates/zed/src/main.rs.orig	2024-10-24 17:42:16 UTC
+++ crates/zed/src/main.rs
@@ -98,12 +98,12 @@ fn fail_to_open_window(e: anyhow::Error, _cx: &mut App
     eprintln!(
         "Zed failed to open a window: {e:?}. See https://zed.dev/docs/linux for troubleshooting steps."
     );
-    #[cfg(not(target_os = "linux"))]
+    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
     {
         process::exit(1);
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     {
         use ashpd::desktop::notification::{Notification, NotificationProxy, Priority};
         _cx.spawn(|_cx| async move {
@@ -237,7 +237,7 @@ fn init_ui(
 
     load_embedded_fonts(cx);
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     crate::zed::linux_prompts::init(cx);
 
     app_state.languages.set_theme(cx.theme().clone());
@@ -353,7 +353,7 @@ fn main() {
 
     let (open_listener, mut open_rx) = OpenListener::new();
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     {
         if env::var("ZED_STATELESS").is_err() {
             if crate::zed::listen_for_cli_connections(open_listener.clone()).is_err() {
@@ -1028,7 +1028,7 @@ fn init_logger() {
                     config_builder.set_time_offset(offset);
                 }
 
-                #[cfg(target_os = "linux")]
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                 {
                     config_builder.add_filter_ignore_str("zbus");
                     config_builder.add_filter_ignore_str("blade_graphics::hal::resource");
