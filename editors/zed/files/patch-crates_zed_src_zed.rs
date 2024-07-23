--- crates/zed/src/zed.rs.orig	2024-07-19 19:12:05 UTC
+++ crates/zed/src/zed.rs
@@ -1,8 +1,8 @@ pub mod inline_completion_registry;
 mod app_menus;
 pub mod inline_completion_registry;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub(crate) mod linux_prompts;
-#[cfg(not(target_os = "linux"))]
+#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
 pub(crate) mod only_instance;
 mod open_listener;
 
@@ -290,7 +290,7 @@ pub fn initialize_workspace(app_state: Arc<AppState>, 
             })
             .register_action(|_, _: &install_cli::Install, cx| {
                 cx.spawn(|workspace, mut cx| async move {
-                    if cfg!(target_os = "linux") {
+                    if cfg!(any(target_os = "linux", target_os = "freebsd")) {
                         let prompt = cx.prompt(
                             PromptLevel::Warning,
                             "Could not install the CLI",
