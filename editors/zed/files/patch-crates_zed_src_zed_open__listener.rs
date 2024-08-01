--- crates/zed/src/zed/open_listener.rs.orig	2024-07-31 16:17:45 UTC
+++ crates/zed/src/zed/open_listener.rs
@@ -271,7 +271,7 @@ impl SshClientDelegate {
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub fn listen_for_cli_connections(opener: OpenListener) -> Result<()> {
     use release_channel::RELEASE_CHANNEL_NAME;
     use std::os::unix::net::UnixDatagram;
