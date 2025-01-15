--- crates/workspace/src/workspace.rs.orig	2025-01-13 22:44:47 UTC
+++ crates/workspace/src/workspace.rs
@@ -3994,7 +3994,7 @@ impl Workspace {
         None
     }
 
-    #[cfg(target_os = "windows")]
+    #[cfg(any(target_os = "windows", target_os = "freebsd"))]
     fn shared_screen_for_peer(
         &self,
         _peer_id: PeerId,
@@ -4004,7 +4004,7 @@ impl Workspace {
         None
     }
 
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     fn shared_screen_for_peer(
         &self,
         peer_id: PeerId,
