--- cargo-crates/libc-0.2.153/src/unix/bsd/mod.rs.orig	2024-05-01 01:37:31 UTC
+++ cargo-crates/libc-0.2.153/src/unix/bsd/mod.rs
@@ -480,6 +480,8 @@ pub const POLLWRBAND: ::c_short = 0x100;
 pub const POLLWRNORM: ::c_short = 0x004;
 pub const POLLRDBAND: ::c_short = 0x080;
 pub const POLLWRBAND: ::c_short = 0x100;
+pub const POLLINIGNEOF: ::c_short = 0x2000;
+pub const POLLRDHUP: ::c_short = 0x4000;
 
 pub const BIOCGBLEN: ::c_ulong = 0x40044266;
 pub const BIOCSBLEN: ::c_ulong = 0xc0044266;
