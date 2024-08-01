--- crates/vim/src/vim.rs.orig	2024-07-31 16:17:45 UTC
+++ crates/vim/src/vim.rs
@@ -558,9 +558,9 @@ impl Vim {
                         cx.write_to_clipboard(content.into());
                     }
                     '*' => {
-                        #[cfg(target_os = "linux")]
+                        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                         cx.write_to_primary(content.into());
-                        #[cfg(not(target_os = "linux"))]
+                        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
                         cx.write_to_clipboard(content.into());
                     }
                     '"' => {
@@ -628,11 +628,11 @@ impl Vim {
             '_' | ':' | '.' | '#' | '=' => None,
             '+' => cx.read_from_clipboard().map(|item| item.into()),
             '*' => {
-                #[cfg(target_os = "linux")]
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                 {
                     cx.read_from_primary().map(|item| item.into())
                 }
-                #[cfg(not(target_os = "linux"))]
+                #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
                 {
                     cx.read_from_clipboard().map(|item| item.into())
                 }
