--- crates/workspace/src/workspace.rs.orig	2025-02-01 17:50:38 UTC
+++ crates/workspace/src/workspace.rs
@@ -4052,7 +4052,7 @@ impl Workspace {
         None
     }
 
-    #[cfg(target_os = "windows")]
+    #[cfg(any(target_os = "windows", target_os = "freebsd"))]
     fn shared_screen_for_peer(
         &self,
         _peer_id: PeerId,
@@ -4062,7 +4062,7 @@ impl Workspace {
         None
     }
 
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     fn shared_screen_for_peer(
         &self,
         peer_id: PeerId,
