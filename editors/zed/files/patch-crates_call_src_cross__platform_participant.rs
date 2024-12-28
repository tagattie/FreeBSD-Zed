--- crates/call/src/cross_platform/participant.rs.orig	2024-12-28 12:36:37 UTC
+++ crates/call/src/cross_platform/participant.rs
@@ -1,4 +1,4 @@
-#![cfg_attr(target_os = "windows", allow(unused))]
+#![cfg_attr(any(target_os = "windows", target_os = "freebsd"), allow(unused))]
 
 use anyhow::{anyhow, Result};
 use client::{proto, ParticipantIndex, User};
@@ -8,7 +8,7 @@ use std::sync::Arc;
 use project::Project;
 use std::sync::Arc;
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 pub use livekit_client::id::TrackSid;
 pub use livekit_client::track::{RemoteAudioTrack, RemoteVideoTrack};
 
@@ -52,17 +52,17 @@ pub struct RemoteParticipant {
     pub participant_index: ParticipantIndex,
     pub muted: bool,
     pub speaking: bool,
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     pub video_tracks: HashMap<TrackSid, RemoteVideoTrack>,
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     pub audio_tracks: HashMap<TrackSid, (RemoteAudioTrack, AudioStream)>,
 }
 
 impl RemoteParticipant {
     pub fn has_video_tracks(&self) -> bool {
-        #[cfg(not(target_os = "windows"))]
+        #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
         return !self.video_tracks.is_empty();
-        #[cfg(target_os = "windows")]
+        #[cfg(any(target_os = "windows", target_os = "freebsd"))]
         return false;
     }
 }
