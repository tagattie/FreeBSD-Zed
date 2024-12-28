--- crates/workspace/src/workspace.rs.orig	2024-12-28 12:39:35 UTC
+++ crates/workspace/src/workspace.rs
@@ -3944,7 +3944,7 @@ impl Workspace {
         None
     }
 
-    #[cfg(target_os = "windows")]
+    #[cfg(any(target_os = "windows", target_os = "freebsd"))]
     fn shared_screen_for_peer(
         &self,
         _peer_id: PeerId,
@@ -3954,7 +3954,7 @@ impl Workspace {
         None
     }
 
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     fn shared_screen_for_peer(
         &self,
         peer_id: PeerId,
