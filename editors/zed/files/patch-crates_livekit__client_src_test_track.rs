--- crates/livekit_client/src/test/track.rs.orig	2024-12-28 12:23:34 UTC
+++ crates/livekit_client/src/test/track.rs
@@ -1,8 +1,8 @@ use super::*;
 use super::*;
-#[cfg(not(windows))]
+#[cfg(not(any(windows, target_os = "freebsd")))]
 use webrtc::{audio_source::RtcAudioSource, video_source::RtcVideoSource};
 
-#[cfg(not(windows))]
+#[cfg(not(any(windows, target_os = "freebsd")))]
 pub use livekit::track::{TrackKind, TrackSource};
 
 #[derive(Clone, Debug)]
@@ -25,14 +25,14 @@ pub struct RemoteVideoTrack {
 
 #[derive(Clone, Debug)]
 pub struct RemoteVideoTrack {
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     pub(super) server_track: Arc<TestServerVideoTrack>,
     pub(super) _room: WeakRoom,
 }
 
 #[derive(Clone, Debug)]
 pub struct RemoteAudioTrack {
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     pub(super) server_track: Arc<TestServerAudioTrack>,
     pub(super) room: WeakRoom,
 }
@@ -43,17 +43,17 @@ pub struct RtcAudioTrack {
 }
 
 pub struct RtcAudioTrack {
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     pub(super) server_track: Arc<TestServerAudioTrack>,
     pub(super) room: WeakRoom,
 }
 
 pub struct RtcVideoTrack {
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     pub(super) _server_track: Arc<TestServerVideoTrack>,
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 impl RemoteTrack {
     pub fn sid(&self) -> TrackSid {
         match self {
@@ -84,21 +84,21 @@ impl RemoteTrack {
     }
 }
 
-#[cfg(not(windows))]
+#[cfg(not(any(windows, target_os = "freebsd")))]
 impl LocalVideoTrack {
     pub fn create_video_track(_name: &str, _source: RtcVideoSource) -> Self {
         Self {}
     }
 }
 
-#[cfg(not(windows))]
+#[cfg(not(any(windows, target_os = "freebsd")))]
 impl LocalAudioTrack {
     pub fn create_audio_track(_name: &str, _source: RtcAudioSource) -> Self {
         Self {}
     }
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 impl RemoteAudioTrack {
     pub fn sid(&self) -> TrackSid {
         self.server_track.sid.clone()
@@ -134,7 +134,7 @@ impl RemoteAudioTrack {
     }
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 impl RemoteVideoTrack {
     pub fn sid(&self) -> TrackSid {
         self.server_track.sid.clone()
@@ -151,7 +151,7 @@ impl RemoteVideoTrack {
     }
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 impl RtcTrack {
     pub fn enabled(&self) -> bool {
         match self {
@@ -168,7 +168,7 @@ impl RtcTrack {
     }
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 impl RtcAudioTrack {
     pub fn set_enabled(&self, enabled: bool) {
         if let Some(room) = self.room.upgrade() {
