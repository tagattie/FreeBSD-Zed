--- crates/util/src/paths.rs.orig	2024-10-16 18:48:23 UTC
+++ crates/util/src/paths.rs
@@ -57,7 +57,7 @@ impl<T: AsRef<Path>> PathExt for T {
     ///   does not have the user's home directory prefix, or if we are not on
     ///   Linux or macOS, the original path is returned unchanged.
     fn compact(&self) -> PathBuf {
-        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
+        if cfg!(target_os = "linux") || cfg!(target_os = "freebsd") || cfg!(target_os = "macos") {
             match self.as_ref().strip_prefix(home_dir().as_path()) {
                 Ok(relative_path) => {
                     let mut shortened_path = PathBuf::new();
@@ -667,7 +667,7 @@ mod tests {
         ]
         .iter()
         .collect();
-        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
+        if cfg!(target_os = "linux") || cfg!(target_os = "freebsd") || cfg!(target_os = "macos") {
             assert_eq!(path.compact().to_str(), Some("~/some_file.txt"));
         } else {
             assert_eq!(path.compact().to_str(), path.to_str());
