--- crates/cli/src/main.rs.orig	2024-07-24 16:08:14 UTC
+++ crates/cli/src/main.rs
@@ -1,4 +1,4 @@
-#![cfg_attr(any(target_os = "linux", target_os = "windows"), allow(dead_code))]
+#![cfg_attr(any(target_os = "linux", target_os = "freebsd", target_os = "windows"), allow(dead_code))]
 
 use anyhow::{Context, Result};
 use clap::Parser;
@@ -178,7 +178,7 @@ fn main() -> Result<()> {
     Ok(())
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod linux {
     use std::{
         env,
