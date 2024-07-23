--- crates/fs/src/fs.rs.orig	2024-07-10 14:12:26 UTC
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
@@ -137,6 +137,8 @@ pub struct RealWatcher {
 pub struct RealWatcher {
     #[cfg(target_os = "linux")]
     fs_watcher: parking_lot::Mutex<notify::INotifyWatcher>,
+    #[cfg(target_os = "freebsd")]
+    fs_watcher: parking_lot::Mutex<notify::KqueueWatcher>,
 }
 
 impl RealFs {
@@ -290,7 +292,7 @@ impl Fs for RealFs {
         Ok(())
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     async fn trash_file(&self, path: &Path, _options: RemoveOptions) -> Result<()> {
         let file = File::open(path)?;
         match trash::trash_file(&file.as_fd()).await {
@@ -304,7 +306,7 @@ impl Fs for RealFs {
         self.trash_file(path, options).await
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     async fn trash_dir(&self, path: &Path, options: RemoveOptions) -> Result<()> {
         self.trash_file(path, options).await
     }
@@ -321,7 +323,7 @@ impl Fs for RealFs {
 
     async fn atomic_write(&self, path: PathBuf, data: String) -> Result<()> {
         smol::unblock(move || {
-            let mut tmp_file = if cfg!(target_os = "linux") {
+            let mut tmp_file = if cfg!(any(target_os = "linux", target_os = "freebsd")) {
                 // Use the directory of the destination as temp dir to avoid
                 // invalid cross-device link error, and XDG_CACHE_DIR for fallback.
                 // See https://github.com/zed-industries/zed/pull/8437 for more details.
@@ -446,7 +448,7 @@ impl Fs for RealFs {
         )
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     async fn watch(
         &self,
         path: &Path,
@@ -601,7 +603,7 @@ impl Fs for RealFs {
     }
 }
 
-#[cfg(not(target_os = "linux"))]
+#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
 impl Watcher for RealWatcher {
     fn add(&self, _: &Path) -> Result<()> {
         Ok(())
@@ -612,7 +614,7 @@ impl Watcher for RealWatcher {
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 impl Watcher for RealWatcher {
     fn add(&self, path: &Path) -> Result<()> {
         use notify::Watcher;
