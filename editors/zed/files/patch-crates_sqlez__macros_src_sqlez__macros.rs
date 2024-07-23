--- crates/sqlez_macros/src/sqlez_macros.rs.orig	2024-07-11 12:13:06 UTC
+++ crates/sqlez_macros/src/sqlez_macros.rs
@@ -12,10 +12,10 @@ pub fn sql(tokens: TokenStream) -> TokenStream {
 pub fn sql(tokens: TokenStream) -> TokenStream {
     let (spans, sql) = make_sql(tokens);
 
-    #[cfg(not(target_os = "linux"))]
+    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
     let error = SQLITE.sql_has_syntax_error(sql.trim());
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     let error: Option<(String, usize)> = None;
 
     let formatted_sql = sqlformat::format(&sql, &sqlformat::QueryParams::None, Default::default());
