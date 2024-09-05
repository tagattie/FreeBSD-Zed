--- crates/sqlez_macros/src/sqlez_macros.rs.orig	2024-09-04 14:30:20 UTC
+++ crates/sqlez_macros/src/sqlez_macros.rs
@@ -1,7 +1,7 @@ use syn::Error;
 use proc_macro::{Delimiter, Span, TokenStream, TokenTree};
 use syn::Error;
 
-#[cfg(not(target_os = "linux"))]
+#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
 static SQLITE: std::sync::LazyLock<sqlez::thread_safe_connection::ThreadSafeConnection> =
     std::sync::LazyLock::new(|| {
         sqlez::thread_safe_connection::ThreadSafeConnection::new(
@@ -16,10 +16,10 @@ pub fn sql(tokens: TokenStream) -> TokenStream {
 pub fn sql(tokens: TokenStream) -> TokenStream {
     let (spans, sql) = make_sql(tokens);
 
-    #[cfg(not(target_os = "linux"))]
+    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
     let error = SQLITE.sql_has_syntax_error(sql.trim());
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     let error: Option<(String, usize)> = None;
 
     let formatted_sql = sqlformat::format(&sql, &sqlformat::QueryParams::None, Default::default());
