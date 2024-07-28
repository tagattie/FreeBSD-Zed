--- crates/gpui/src/platform.rs.orig	2024-07-24 16:08:14 UTC
+++ crates/gpui/src/platform.rs
@@ -7,13 +7,13 @@ mod cosmic_text;
 #[cfg(not(target_os = "macos"))]
 mod cosmic_text;
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod linux;
 
 #[cfg(target_os = "macos")]
 mod mac;
 
-#[cfg(any(target_os = "linux", target_os = "windows", feature = "macos-blade"))]
+#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "windows", feature = "macos-blade"))]
 mod blade;
 
 #[cfg(any(test, feature = "test-support"))]
@@ -53,7 +53,7 @@ pub(crate) use cosmic_text::*;
 
 #[cfg(not(target_os = "macos"))]
 pub(crate) use cosmic_text::*;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub(crate) use linux::*;
 #[cfg(target_os = "macos")]
 pub(crate) use mac::*;
@@ -68,7 +68,7 @@ pub(crate) fn current_platform() -> Rc<dyn Platform> {
 pub(crate) fn current_platform() -> Rc<dyn Platform> {
     Rc::new(MacPlatform::new())
 }
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub(crate) fn current_platform() -> Rc<dyn Platform> {
     match guess_compositor() {
         "Wayland" => Rc::new(WaylandClient::new()),
@@ -80,7 +80,7 @@ pub(crate) fn current_platform() -> Rc<dyn Platform> {
 
 /// Return which compositor we're guessing we'll use.
 /// Does not attempt to connect to the given compositor
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 #[inline]
 pub fn guess_compositor() -> &'static str {
     let wayland_display = std::env::var_os("WAYLAND_DISPLAY");
@@ -165,10 +165,10 @@ pub(crate) trait Platform: 'static {
     fn set_cursor_style(&self, style: CursorStyle);
     fn should_auto_hide_scrollbars(&self) -> bool;
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     fn write_to_primary(&self, item: ClipboardItem);
     fn write_to_clipboard(&self, item: ClipboardItem);
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     fn read_from_primary(&self) -> Option<ClipboardItem>;
     fn read_from_clipboard(&self) -> Option<ClipboardItem>;
 
@@ -524,7 +524,7 @@ impl PlatformInputHandler {
             .flatten()
     }
 
-    #[cfg_attr(target_os = "linux", allow(dead_code))]
+    #[cfg_attr(any(target_os = "linux", target_os = "freebsd"), allow(dead_code))]
     fn text_for_range(&mut self, range_utf16: Range<usize>) -> Option<String> {
         self.cx
             .update(|cx| self.handler.text_for_range(range_utf16, cx))
@@ -692,22 +692,22 @@ pub(crate) struct WindowParams {
     pub titlebar: Option<TitlebarOptions>,
 
     /// The kind of window to create
-    #[cfg_attr(target_os = "linux", allow(dead_code))]
+    #[cfg_attr(any(target_os = "linux", target_os = "freebsd"), allow(dead_code))]
     pub kind: WindowKind,
 
     /// Whether the window should be movable by the user
-    #[cfg_attr(target_os = "linux", allow(dead_code))]
+    #[cfg_attr(any(target_os = "linux", target_os = "freebsd"), allow(dead_code))]
     pub is_movable: bool,
 
-    #[cfg_attr(target_os = "linux", allow(dead_code))]
+    #[cfg_attr(any(target_os = "linux", target_os = "freebsd"), allow(dead_code))]
     pub focus: bool,
 
-    #[cfg_attr(target_os = "linux", allow(dead_code))]
+    #[cfg_attr(any(target_os = "linux", target_os = "freebsd"), allow(dead_code))]
     pub show: bool,
 
     pub display_id: Option<DisplayId>,
 
-    #[cfg_attr(target_os = "linux", allow(dead_code))]
+    #[cfg_attr(any(target_os = "linux", target_os = "freebsd"), allow(dead_code))]
     pub window_min_size: Option<Size<Pixels>>,
 }
 
@@ -995,7 +995,7 @@ impl ClipboardItem {
             .and_then(|m| serde_json::from_str(m).ok())
     }
 
-    #[cfg_attr(target_os = "linux", allow(dead_code))]
+    #[cfg_attr(any(target_os = "linux", target_os = "freebsd"), allow(dead_code))]
     pub(crate) fn text_hash(text: &str) -> u64 {
         let mut hasher = SeaHasher::new();
         text.hash(&mut hasher);
