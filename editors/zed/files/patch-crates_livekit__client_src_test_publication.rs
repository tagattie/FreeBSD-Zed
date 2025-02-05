--- crates/livekit_client/src/test/publication.rs.orig	2025-02-05 13:37:34 UTC
+++ crates/livekit_client/src/test/publication.rs
@@ -8,17 +8,20 @@ pub struct LocalTrackPublication {
 
 #[derive(Clone, Debug)]
 pub struct LocalTrackPublication {
+    #[cfg(not(target_os = "freebsd"))]
     pub(crate) sid: TrackSid,
     pub(crate) room: WeakRoom,
 }
 
 #[derive(Clone, Debug)]
 pub struct RemoteTrackPublication {
+    #[cfg(not(target_os = "freebsd"))]
     pub(crate) sid: TrackSid,
     pub(crate) room: WeakRoom,
     pub(crate) track: RemoteTrack,
 }
 
+#[cfg(not(target_os = "freebsd"))]
 impl TrackPublication {
     pub fn sid(&self) -> TrackSid {
         match self {
@@ -35,6 +38,7 @@ impl TrackPublication {
     }
 }
 
+#[cfg(not(target_os = "freebsd"))]
 impl LocalTrackPublication {
     pub fn sid(&self) -> TrackSid {
         self.sid.clone()
@@ -67,6 +71,7 @@ impl LocalTrackPublication {
     }
 }
 
+#[cfg(not(target_os = "freebsd"))]
 impl RemoteTrackPublication {
     pub fn sid(&self) -> TrackSid {
         self.sid.clone()
