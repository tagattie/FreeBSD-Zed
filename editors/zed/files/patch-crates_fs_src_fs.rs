--- crates/fs/src/fs.rs.orig	2025-01-13 22:44:47 UTC
+++ crates/fs/src/fs.rs
@@ -269,7 +269,7 @@ impl FileHandle for std::fs::File {
         Ok(path)
     }
 
-    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
+    #[cfg(target_os = "linux")]
     fn current_path(&self, _: &Arc<dyn Fs>) -> Result<PathBuf> {
         let fd = self.as_fd();
         let fd_path = format!("/proc/self/fd/{}", fd.as_raw_fd());
@@ -282,6 +282,27 @@ impl FileHandle for std::fs::File {
         };
 
         Ok(new_path)
+    }
+
+    #[cfg(target_os = "freebsd")]
+    fn current_path(&self, _: &Arc<dyn Fs>) -> Result<PathBuf> {
+        use std::{
+            ffi::{CStr, OsStr},
+            os::unix::ffi::OsStrExt,
+        };
+
+        let fd = self.as_fd();
+        let mut kif: libc::kinfo_file = unsafe { std::mem::zeroed() };
+        kif.kf_structsize = libc::KINFO_FILE_SIZE;
+
+        let result = unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_KINFO, &mut kif) };
+        if result == -1 {
+            anyhow::bail!("fcntl returned -1".to_string());
+        }
+
+        let c_str = unsafe { CStr::from_ptr(kif.kf_path.as_ptr()) };
+        let path = PathBuf::from(OsStr::from_bytes(c_str.to_bytes()));
+        Ok(path)
     }
 
     #[cfg(target_os = "windows")]
