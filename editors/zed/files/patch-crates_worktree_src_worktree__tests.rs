--- crates/worktree/src/worktree_tests.rs.orig	2024-08-01 08:51:53 UTC
+++ crates/worktree/src/worktree_tests.rs
@@ -842,7 +842,7 @@ async fn test_write_file(cx: &mut TestAppContext) {
     .await
     .unwrap();
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     fs::watcher::global(|_| {}).unwrap();
 
     cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
