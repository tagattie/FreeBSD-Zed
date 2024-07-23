--- crates/sqlez/src/connection.rs.orig	2024-07-11 12:11:36 UTC
+++ crates/sqlez/src/connection.rs
@@ -128,10 +128,10 @@ impl Connection {
                             &mut remaining_sql_ptr,
                         );
 
-                        #[cfg(not(target_os = "linux"))]
+                        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
                         let offset = sqlite3_error_offset(temp_connection.sqlite3);
 
-                        #[cfg(target_os = "linux")]
+                        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                         let offset = 0;
 
                         (
@@ -149,10 +149,10 @@ impl Connection {
                             &mut remaining_sql_ptr,
                         );
 
-                        #[cfg(not(target_os = "linux"))]
+                        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
                         let offset = sqlite3_error_offset(self.sqlite3);
 
-                        #[cfg(target_os = "linux")]
+                        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                         let offset = 0;
 
                         (
@@ -408,7 +408,7 @@ mod test {
         );
     }
 
-    #[cfg(not(target_os = "linux"))]
+    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
     #[test]
     fn test_sql_has_syntax_errors() {
         let connection = Connection::open_memory(Some("test_sql_has_syntax_errors"));
