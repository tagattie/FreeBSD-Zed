--- crates/extension/src/extension_builder.rs.orig	2024-12-11 11:14:59 UTC
+++ crates/extension/src/extension_builder.rs
@@ -36,7 +36,7 @@ const WASI_SDK_ASSET_NAME: Option<&str> = if cfg!(targ
 const WASI_SDK_URL: &str = "https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-21/";
 const WASI_SDK_ASSET_NAME: Option<&str> = if cfg!(target_os = "macos") {
     Some("wasi-sdk-21.0-macos.tar.gz")
-} else if cfg!(any(target_os = "linux", target_os = "freebsd")) {
+} else if cfg!(target_os = "linux") {
     Some("wasi-sdk-21.0-linux.tar.gz")
 } else if cfg!(target_os = "windows") {
     Some("wasi-sdk-21.0.m-mingw.tar.gz")
