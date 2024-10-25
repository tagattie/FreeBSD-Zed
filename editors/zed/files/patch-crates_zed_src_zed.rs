--- crates/zed/src/zed.rs.orig	2024-10-24 17:42:16 UTC
+++ crates/zed/src/zed.rs
@@ -1,6 +1,6 @@ pub mod inline_completion_registry;
 mod app_menus;
 pub mod inline_completion_registry;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub(crate) mod linux_prompts;
 #[cfg(target_os = "macos")]
 pub(crate) mod mac_only_instance;
@@ -151,7 +151,7 @@ pub fn initialize_workspace(
         })
         .detach();
 
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         if let Err(e) = fs::watcher::global(|_| {}) {
             let message = format!(db::indoc!{r#"
                 inotify_init returned {}
@@ -323,7 +323,7 @@ pub fn initialize_workspace(
             })
             .register_action(|_, _: &install_cli::Install, cx| {
                 cx.spawn(|workspace, mut cx| async move {
-                    if cfg!(target_os = "linux") {
+                    if cfg!(any(target_os = "linux", target_os = "freebsd")) {
                         let prompt = cx.prompt(
                             PromptLevel::Warning,
                             "CLI should already be installed",
