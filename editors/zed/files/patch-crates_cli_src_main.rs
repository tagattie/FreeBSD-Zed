--- crates/cli/src/main.rs.orig	2024-08-27 11:06:16 UTC
+++ crates/cli/src/main.rs
@@ -1,4 +1,4 @@
-#![cfg_attr(any(target_os = "linux", target_os = "windows"), allow(dead_code))]
+#![cfg_attr(any(target_os = "linux", target_os = "freebsd", target_os = "windows"), allow(dead_code))]
 
 use anyhow::{Context, Result};
 use clap::Parser;
@@ -175,7 +175,7 @@ fn main() -> Result<()> {
     Ok(())
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod linux {
     use std::{
         env,
