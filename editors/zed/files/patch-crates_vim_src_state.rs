--- crates/vim/src/state.rs.orig	2024-09-04 21:29:55 UTC
+++ crates/vim/src/state.rs
@@ -225,9 +225,9 @@ impl VimGlobals {
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
@@ -295,11 +295,11 @@ impl VimGlobals {
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
