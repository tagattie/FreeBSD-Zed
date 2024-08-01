--- crates/terminal/src/terminal.rs.orig	2024-07-31 16:17:45 UTC
+++ crates/terminal/src/terminal.rs
@@ -346,10 +346,10 @@ impl TerminalBuilder {
             alacritty_terminal::tty::Options {
                 shell: alac_shell,
                 working_directory: working_directory.clone(),
-                #[cfg(target_os = "linux")]
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                 hold: !matches!(shell.clone(), Shell::System),
                 // with hold: true, macOS gets tasks stuck on ctrl-c interrupts periodically
-                #[cfg(not(target_os = "linux"))]
+                #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
                 hold: false,
                 env: env.into_iter().collect(),
             }
@@ -762,7 +762,7 @@ impl Terminal {
             InternalEvent::SetSelection(selection) => {
                 term.selection = selection.as_ref().map(|(sel, _)| sel.clone());
 
-                #[cfg(target_os = "linux")]
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                 if let Some(selection_text) = term.selection_to_string() {
                     cx.write_to_primary(ClipboardItem::new(selection_text));
                 }
@@ -783,7 +783,7 @@ impl Terminal {
                     selection.update(point, side);
                     term.selection = Some(selection);
 
-                    #[cfg(target_os = "linux")]
+                    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                     if let Some(selection_text) = term.selection_to_string() {
                         cx.write_to_primary(ClipboardItem::new(selection_text));
                     }
@@ -1336,7 +1336,7 @@ impl Terminal {
                             .push_back(InternalEvent::SetSelection(Some((sel, point))));
                     }
                 }
-                #[cfg(target_os = "linux")]
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                 MouseButton::Middle => {
                     if let Some(item) = _cx.read_from_primary() {
                         let text = item.text().to_string();
