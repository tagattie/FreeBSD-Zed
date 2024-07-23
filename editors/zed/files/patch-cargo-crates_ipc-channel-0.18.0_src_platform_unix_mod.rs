--- cargo-crates/ipc-channel-0.18.0/src/platform/unix/mod.rs.orig	2024-05-01 01:41:55 UTC
+++ cargo-crates/ipc-channel-0.18.0/src/platform/unix/mod.rs
@@ -1167,7 +1167,7 @@ impl UnixCmsg {
                 }];
                 let result = libc::poll(
                     fd.as_mut_ptr(),
-                    fd.len() as libc::c_ulong,
+                    fd.len() as libc::c_uint,
                     duration.as_millis().try_into().unwrap_or(-1),
                 );
 
