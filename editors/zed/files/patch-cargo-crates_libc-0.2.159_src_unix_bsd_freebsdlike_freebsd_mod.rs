--- cargo-crates/libc-0.2.159/src/unix/bsd/freebsdlike/freebsd/mod.rs.orig	2024-10-21 20:22:27 UTC
+++ cargo-crates/libc-0.2.159/src/unix/bsd/freebsdlike/freebsd/mod.rs
@@ -2894,6 +2894,7 @@ pub const POLLINIGNEOF: ::c_short = 0x2000;
 pub const POSIX_FADV_NOREUSE: ::c_int = 5;
 
 pub const POLLINIGNEOF: ::c_short = 0x2000;
+pub const POLLRDHUP: ::c_short = 0x4000;
 
 pub const EVFILT_READ: i16 = -1;
 pub const EVFILT_WRITE: i16 = -2;
