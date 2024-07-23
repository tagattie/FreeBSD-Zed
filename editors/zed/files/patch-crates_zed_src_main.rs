--- crates/zed/src/main.rs.orig	2024-07-19 19:12:05 UTC
+++ crates/zed/src/main.rs
@@ -83,12 +83,12 @@ fn fail_to_open_window(e: anyhow::Error, _cx: &mut App
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
@@ -175,7 +175,7 @@ fn init_ui(app_state: Arc<AppState>, cx: &mut AppConte
     SystemAppearance::init(cx);
     load_embedded_fonts(cx);
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     crate::zed::linux_prompts::init(cx);
 
     theme::init(theme::LoadThemes::All(Box::new(Assets)), cx);
@@ -315,7 +315,7 @@ fn main() {
 
     let (open_listener, mut open_rx) = OpenListener::new();
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     {
         if env::var("ZED_STATELESS").is_err() {
             if crate::zed::listen_for_cli_connections(open_listener.clone()).is_err() {
@@ -324,7 +324,7 @@ fn main() {
             }
         }
     }
-    #[cfg(not(target_os = "linux"))]
+    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
     {
         use zed::only_instance::*;
         if ensure_only_instance() != IsOnlyInstance::Yes {
@@ -718,7 +718,7 @@ fn init_logger() {
                 config_builder.set_time_format_rfc3339();
                 config_builder.set_time_offset_to_local().log_err();
 
-                #[cfg(target_os = "linux")]
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                 {
                     config_builder.add_filter_ignore_str("zbus");
                     config_builder.add_filter_ignore_str("blade_graphics::hal::resource");
