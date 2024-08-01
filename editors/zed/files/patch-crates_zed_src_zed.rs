--- crates/zed/src/zed.rs.orig	2024-07-31 16:17:45 UTC
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
 pub(crate) mod session;
@@ -140,7 +140,7 @@ pub fn initialize_workspace(app_state: Arc<AppState>, 
         })
         .detach();
 
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         if let Err(e) = fs::watcher::global(|_| {}) {
             let message = format!(db::indoc!{r#"
                 inotify_init returned {}
@@ -330,7 +330,7 @@ pub fn initialize_workspace(app_state: Arc<AppState>, 
             })
             .register_action(|_, _: &install_cli::Install, cx| {
                 cx.spawn(|workspace, mut cx| async move {
-                    if cfg!(target_os = "linux") {
+                    if cfg!(any(target_os = "linux", target_os = "freebsd")) {
                         let prompt = cx.prompt(
                             PromptLevel::Warning,
                             "CLI should already be installed",
