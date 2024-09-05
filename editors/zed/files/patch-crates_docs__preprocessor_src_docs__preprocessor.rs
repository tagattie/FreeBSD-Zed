--- crates/docs_preprocessor/src/docs_preprocessor.rs.orig	2024-09-04 21:22:28 UTC
+++ crates/docs_preprocessor/src/docs_preprocessor.rs
@@ -28,7 +28,7 @@ impl PreprocessorContext {
     pub fn find_binding(&self, os: &str, action: &str) -> Option<String> {
         let keymap = match os {
             "macos" => &self.macos_keymap,
-            "linux" => &self.linux_keymap,
+            "linux" | "freebsd" => &self.linux_keymap,
             _ => return None,
         };
 
