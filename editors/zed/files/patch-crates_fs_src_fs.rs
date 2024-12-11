--- crates/fs/src/fs.rs.orig	2024-12-04 17:01:03 UTC
+++ crates/fs/src/fs.rs
@@ -267,7 +267,7 @@ impl FileHandle for std::fs::File {
         Ok(path)
     }
 
-    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
+    #[cfg(target_os = "linux")]
     fn current_path(&self, _: &Arc<dyn Fs>) -> Result<PathBuf> {
         let fd = self.as_fd();
         let fd_path = format!("/proc/self/fd/{}", fd.as_raw_fd());
@@ -282,7 +282,7 @@ impl FileHandle for std::fs::File {
         Ok(new_path)
     }
 
-    #[cfg(target_os = "windows")]
+    #[cfg(any(target_os = "windows", target_os = "freebsd"))]
     fn current_path(&self, _: &Arc<dyn Fs>) -> Result<PathBuf> {
         anyhow::bail!("unimplemented")
     }
