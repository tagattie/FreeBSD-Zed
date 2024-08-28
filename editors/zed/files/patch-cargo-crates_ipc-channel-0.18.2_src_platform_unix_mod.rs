--- cargo-crates/ipc-channel-0.18.2/src/platform/unix/mod.rs.orig	2024-08-28 07:10:54 UTC
+++ cargo-crates/ipc-channel-0.18.2/src/platform/unix/mod.rs
@@ -1170,7 +1170,7 @@ impl UnixCmsg {
                 }];
                 let result = libc::poll(
                     fd.as_mut_ptr(),
-                    fd.len() as libc::c_ulong,
+                    fd.len() as libc::c_uint,
                     duration.as_millis().try_into().unwrap_or(-1),
                 );
 
