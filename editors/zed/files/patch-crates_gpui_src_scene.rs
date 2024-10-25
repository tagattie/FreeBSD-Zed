--- crates/gpui/src/scene.rs.orig	2024-10-25 11:25:04 UTC
+++ crates/gpui/src/scene.rs
@@ -41,7 +41,7 @@ impl Scene {
     }
 
     #[cfg_attr(
-        all(target_os = "linux", not(any(feature = "x11", feature = "wayland"))),
+        all(any(target_os = "linux", target_os = "freebsd"), not(any(feature = "x11", feature = "wayland"))),
         allow(dead_code)
     )]
     pub fn paths(&self) -> &[Path<ScaledPixels>] {
@@ -135,7 +135,7 @@ impl Scene {
     }
 
     #[cfg_attr(
-        all(target_os = "linux", not(any(feature = "x11", feature = "wayland"))),
+        all(any(target_os = "linux", target_os = "freebsd"), not(any(feature = "x11", feature = "wayland"))),
         allow(dead_code)
     )]
     pub(crate) fn batches(&self) -> impl Iterator<Item = PrimitiveBatch> {
@@ -167,7 +167,7 @@ impl Scene {
 
 #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
 #[cfg_attr(
-    all(target_os = "linux", not(any(feature = "x11", feature = "wayland"))),
+    all(any(target_os = "linux", target_os = "freebsd"), not(any(feature = "x11", feature = "wayland"))),
     allow(dead_code)
 )]
 pub(crate) enum PrimitiveKind {
@@ -225,7 +225,7 @@ impl Primitive {
 }
 
 #[cfg_attr(
-    all(target_os = "linux", not(any(feature = "x11", feature = "wayland"))),
+    all(any(target_os = "linux", target_os = "freebsd"), not(any(feature = "x11", feature = "wayland"))),
     allow(dead_code)
 )]
 struct BatchIterator<'a> {
@@ -415,7 +415,7 @@ impl<'a> Iterator for BatchIterator<'a> {
 
 #[derive(Debug)]
 #[cfg_attr(
-    all(target_os = "linux", not(any(feature = "x11", feature = "wayland"))),
+    all(any(target_os = "linux", target_os = "freebsd"), not(any(feature = "x11", feature = "wayland"))),
     allow(dead_code)
 )]
 pub(crate) enum PrimitiveBatch<'a> {
