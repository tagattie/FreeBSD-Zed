--- crates/terminal/src/terminal.rs.orig	2024-10-24 17:42:16 UTC
+++ crates/terminal/src/terminal.rs
@@ -806,7 +806,7 @@ impl Terminal {
                         selection.update(point, AlacDirection::Right);
                         term.selection = Some(selection);
 
-                        #[cfg(target_os = "linux")]
+                        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                         if let Some(selection_text) = term.selection_to_string() {
                             cx.write_to_primary(ClipboardItem::new_string(selection_text));
                         }
@@ -819,7 +819,7 @@ impl Terminal {
             InternalEvent::SetSelection(selection) => {
                 term.selection = selection.as_ref().map(|(sel, _)| sel.clone());
 
-                #[cfg(target_os = "linux")]
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                 if let Some(selection_text) = term.selection_to_string() {
                     cx.write_to_primary(ClipboardItem::new_string(selection_text));
                 }
@@ -840,7 +840,7 @@ impl Terminal {
                     selection.update(point, side);
                     term.selection = Some(selection);
 
-                    #[cfg(target_os = "linux")]
+                    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                     if let Some(selection_text) = term.selection_to_string() {
                         cx.write_to_primary(ClipboardItem::new_string(selection_text));
                     }
@@ -1496,7 +1496,7 @@ impl Terminal {
                             .push_back(InternalEvent::SetSelection(Some((sel, point))));
                     }
                 }
-                #[cfg(target_os = "linux")]
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                 MouseButton::Middle => {
                     if let Some(item) = _cx.read_from_primary() {
                         let text = item.text().unwrap_or_default().to_string();
