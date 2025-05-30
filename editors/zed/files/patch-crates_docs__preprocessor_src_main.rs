--- crates/docs_preprocessor/src/main.rs.orig	2025-05-30 21:20:44 UTC
+++ crates/docs_preprocessor/src/main.rs
@@ -118,7 +118,7 @@ fn find_binding(os: &str, action: &str) -> Option<Stri
 fn find_binding(os: &str, action: &str) -> Option<String> {
     let keymap = match os {
         "macos" => &KEYMAP_MACOS,
-        "linux" => &KEYMAP_LINUX,
+        "linux" | "freebsd" => &KEYMAP_LINUX,
         _ => unreachable!("Not a valid OS: {}", os),
     };
 
