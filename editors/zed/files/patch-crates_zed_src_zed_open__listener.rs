--- crates/zed/src/zed/open_listener.rs.orig	2024-08-27 11:06:16 UTC
+++ crates/zed/src/zed/open_listener.rs
@@ -137,7 +137,7 @@ impl OpenListener {
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub fn listen_for_cli_connections(opener: OpenListener) -> Result<()> {
     use release_channel::RELEASE_CHANNEL_NAME;
     use std::os::unix::net::UnixDatagram;
