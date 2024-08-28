--- cargo-crates/libc-0.2.155/src/unix/bsd/freebsdlike/freebsd/mod.rs.orig	2024-08-28 07:12:44 UTC
+++ cargo-crates/libc-0.2.155/src/unix/bsd/freebsdlike/freebsd/mod.rs
@@ -2708,6 +2708,7 @@ pub const POLLINIGNEOF: ::c_short = 0x2000;
 pub const POSIX_FADV_NOREUSE: ::c_int = 5;
 
 pub const POLLINIGNEOF: ::c_short = 0x2000;
+pub const POLLRDHUP: ::c_short = 0x4000;
 
 pub const EVFILT_READ: i16 = -1;
 pub const EVFILT_WRITE: i16 = -2;
