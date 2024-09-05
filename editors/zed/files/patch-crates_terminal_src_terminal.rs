--- crates/terminal/src/terminal.rs.orig	2024-09-04 14:30:20 UTC
+++ crates/terminal/src/terminal.rs
@@ -768,7 +768,7 @@ impl Terminal {
             InternalEvent::SetSelection(selection) => {
                 term.selection = selection.as_ref().map(|(sel, _)| sel.clone());
 
-                #[cfg(target_os = "linux")]
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                 if let Some(selection_text) = term.selection_to_string() {
                     cx.write_to_primary(ClipboardItem::new_string(selection_text));
                 }
@@ -789,7 +789,7 @@ impl Terminal {
                     selection.update(point, side);
                     term.selection = Some(selection);
 
-                    #[cfg(target_os = "linux")]
+                    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                     if let Some(selection_text) = term.selection_to_string() {
                         cx.write_to_primary(ClipboardItem::new_string(selection_text));
                     }
@@ -1331,7 +1331,7 @@ impl Terminal {
                             .push_back(InternalEvent::SetSelection(Some((sel, point))));
                     }
                 }
-                #[cfg(target_os = "linux")]
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                 MouseButton::Middle => {
                     if let Some(item) = _cx.read_from_primary() {
                         let text = item.text().unwrap_or_default().to_string();
