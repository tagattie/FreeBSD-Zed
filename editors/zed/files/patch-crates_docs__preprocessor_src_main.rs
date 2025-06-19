--- crates/docs_preprocessor/src/main.rs.orig	2025-06-18 19:13:38 UTC
+++ crates/docs_preprocessor/src/main.rs
@@ -171,7 +171,7 @@ fn find_binding(os: &str, action: &str) -> Option<Stri
 fn find_binding(os: &str, action: &str) -> Option<String> {
     let keymap = match os {
         "macos" => &KEYMAP_MACOS,
-        "linux" => &KEYMAP_LINUX,
+        "linux" | "freebsd" => &KEYMAP_LINUX,
         _ => unreachable!("Not a valid OS: {}", os),
     };
 
