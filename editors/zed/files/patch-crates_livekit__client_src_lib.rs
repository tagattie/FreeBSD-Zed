--- crates/livekit_client/src/lib.rs.orig	2025-05-15 07:31:14 UTC
+++ crates/livekit_client/src/lib.rs
@@ -6,32 +6,32 @@ pub use remote_video_track_view::{RemoteVideoTrackView
 #[cfg(not(any(
     test,
     feature = "test-support",
-    all(target_os = "windows", target_env = "gnu")
+    any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")
 )))]
 mod livekit_client;
 #[cfg(not(any(
     test,
     feature = "test-support",
-    all(target_os = "windows", target_env = "gnu")
+    any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")
 )))]
 pub use livekit_client::*;
 
 #[cfg(any(
     test,
     feature = "test-support",
-    all(target_os = "windows", target_env = "gnu")
+    any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")
 ))]
 mod mock_client;
 #[cfg(any(
     test,
     feature = "test-support",
-    all(target_os = "windows", target_env = "gnu")
+    any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")
 ))]
 pub mod test;
 #[cfg(any(
     test,
     feature = "test-support",
-    all(target_os = "windows", target_env = "gnu")
+    any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")
 ))]
 pub use mock_client::*;
 
