--- crates/gpui/src/platform/test/platform.rs.orig	2024-07-11 12:06:02 UTC
+++ crates/gpui/src/platform/test/platform.rs
@@ -23,7 +23,7 @@ pub(crate) struct TestPlatform {
     active_display: Rc<dyn PlatformDisplay>,
     active_cursor: Mutex<CursorStyle>,
     current_clipboard_item: Mutex<Option<ClipboardItem>>,
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     current_primary_item: Mutex<Option<ClipboardItem>>,
     pub(crate) prompts: RefCell<TestPrompts>,
     pub opened_url: RefCell<Option<String>>,
@@ -48,7 +48,7 @@ impl TestPlatform {
         #[cfg(target_os = "macos")]
         let text_system = Arc::new(crate::platform::mac::MacTextSystem::new());
 
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         let text_system = Arc::new(crate::platform::cosmic_text::CosmicTextSystem::new());
 
         #[cfg(target_os = "windows")]
@@ -62,7 +62,7 @@ impl TestPlatform {
             active_display: Rc::new(TestDisplay::new()),
             active_window: Default::default(),
             current_clipboard_item: Mutex::new(None),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             current_primary_item: Mutex::new(None),
             weak: weak.clone(),
             opened_url: Default::default(),
@@ -273,7 +273,7 @@ impl Platform for TestPlatform {
         false
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     fn write_to_primary(&self, item: ClipboardItem) {
         *self.current_primary_item.lock() = Some(item);
     }
@@ -282,7 +282,7 @@ impl Platform for TestPlatform {
         *self.current_clipboard_item.lock() = Some(item);
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     fn read_from_primary(&self) -> Option<ClipboardItem> {
         self.current_primary_item.lock().clone()
     }
