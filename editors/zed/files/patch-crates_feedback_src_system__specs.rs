--- crates/feedback/src/system_specs.rs.orig	2025-06-04 14:12:33 UTC
+++ crates/feedback/src/system_specs.rs
@@ -123,7 +123,7 @@ fn try_determine_available_gpus() -> Option<String> {
 }
 
 fn try_determine_available_gpus() -> Option<String> {
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     {
         return std::process::Command::new("vulkaninfo")
             .args(&["--summary"])
@@ -142,7 +142,7 @@ fn try_determine_available_gpus() -> Option<String> {
             })
             .or(Some("Failed to run `vulkaninfo --summary`".to_string()));
     }
-    #[cfg(not(target_os = "linux"))]
+    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
     {
         return None;
     }
