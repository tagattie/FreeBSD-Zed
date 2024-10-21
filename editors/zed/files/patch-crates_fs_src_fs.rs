--- crates/fs/src/fs.rs.orig	2024-10-16 18:48:23 UTC
+++ crates/fs/src/fs.rs
@@ -1,9 +1,9 @@ use git::GitHostingProviderRegistry;
 use anyhow::{anyhow, Result};
 use git::GitHostingProviderRegistry;
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 use ashpd::desktop::trash;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 use std::{fs::File, os::fd::AsFd};
 
 #[cfg(unix)]
@@ -334,7 +334,7 @@ impl Fs for RealFs {
         Ok(())
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     async fn trash_file(&self, path: &Path, _options: RemoveOptions) -> Result<()> {
         let file = File::open(path)?;
         match trash::trash_file(&file.as_fd()).await {
@@ -366,7 +366,7 @@ impl Fs for RealFs {
         self.trash_file(path, options).await
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     async fn trash_dir(&self, path: &Path, options: RemoveOptions) -> Result<()> {
         self.trash_file(path, options).await
     }
@@ -407,7 +407,7 @@ impl Fs for RealFs {
 
     async fn atomic_write(&self, path: PathBuf, data: String) -> Result<()> {
         smol::unblock(move || {
-            let mut tmp_file = if cfg!(target_os = "linux") {
+            let mut tmp_file = if cfg!(any(target_os = "linux", target_os = "freebsd")) {
                 // Use the directory of the destination as temp dir to avoid
                 // invalid cross-device link error, and XDG_CACHE_DIR for fallback.
                 // See https://github.com/zed-industries/zed/pull/8437 for more details.
@@ -571,7 +571,7 @@ impl Fs for RealFs {
         )
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     async fn watch(
         &self,
         path: &Path,
@@ -773,7 +773,7 @@ impl Fs for RealFs {
     }
 }
 
-#[cfg(not(target_os = "linux"))]
+#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
 impl Watcher for RealWatcher {
     fn add(&self, _: &Path) -> Result<()> {
         Ok(())
@@ -784,7 +784,7 @@ impl Watcher for RealWatcher {
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 impl Watcher for RealWatcher {
     fn add(&self, path: &Path) -> Result<()> {
         use notify::Watcher;
@@ -2028,7 +2028,7 @@ mod tests {
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub mod watcher {
     use std::sync::OnceLock;
 
@@ -2037,7 +2037,10 @@ pub mod watcher {
 
     pub struct GlobalWatcher {
         // two mutexes because calling inotify.add triggers an inotify.event, which needs watchers.
+        #[cfg(target_os = "linux")]
         pub(super) inotify: Mutex<notify::INotifyWatcher>,
+        #[cfg(target_os = "freebsd")]
+        pub(super) inotify: Mutex<notify::KqueueWatcher>,
         pub(super) watchers: Mutex<Vec<Box<dyn Fn(&notify::Event) + Send + Sync>>>,
     }
 
