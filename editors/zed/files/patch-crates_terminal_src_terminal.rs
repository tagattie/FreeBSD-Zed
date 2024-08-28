--- crates/terminal/src/terminal.rs.orig	2024-08-27 11:06:16 UTC
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
@@ -769,7 +769,7 @@ impl Terminal {
             InternalEvent::SetSelection(selection) => {
                 term.selection = selection.as_ref().map(|(sel, _)| sel.clone());
 
-                #[cfg(target_os = "linux")]
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                 if let Some(selection_text) = term.selection_to_string() {
                     cx.write_to_primary(ClipboardItem::new_string(selection_text));
                 }
@@ -790,7 +790,7 @@ impl Terminal {
                     selection.update(point, side);
                     term.selection = Some(selection);
 
-                    #[cfg(target_os = "linux")]
+                    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                     if let Some(selection_text) = term.selection_to_string() {
                         cx.write_to_primary(ClipboardItem::new_string(selection_text));
                     }
@@ -1343,7 +1343,7 @@ impl Terminal {
                             .push_back(InternalEvent::SetSelection(Some((sel, point))));
                     }
                 }
-                #[cfg(target_os = "linux")]
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                 MouseButton::Middle => {
                     if let Some(item) = _cx.read_from_primary() {
                         let text = item.text().unwrap_or_default().to_string();
