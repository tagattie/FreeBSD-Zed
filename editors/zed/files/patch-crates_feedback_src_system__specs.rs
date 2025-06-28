--- crates/feedback/src/system_specs.rs.orig	2025-06-27 16:44:45 UTC
+++ crates/feedback/src/system_specs.rs
@@ -133,7 +133,7 @@ fn try_determine_available_gpus() -> Option<String> {
 }
 
 fn try_determine_available_gpus() -> Option<String> {
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     {
         return std::process::Command::new("vulkaninfo")
             .args(&["--summary"])
@@ -152,7 +152,7 @@ fn try_determine_available_gpus() -> Option<String> {
             })
             .or(Some("Failed to run `vulkaninfo --summary`".to_string()));
     }
-    #[cfg(not(target_os = "linux"))]
+    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
     {
         return None;
     }
