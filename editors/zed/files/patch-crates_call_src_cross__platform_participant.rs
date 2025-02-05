--- crates/call/src/cross_platform/participant.rs.orig	2025-02-05 14:08:53 UTC
+++ crates/call/src/cross_platform/participant.rs
@@ -1,3 +1,5 @@
+#![cfg_attr(target_os = "freebsd", allow(unused))]
+
 use anyhow::{anyhow, Result};
 use client::{proto, ParticipantIndex, User};
 use collections::HashMap;
@@ -6,6 +8,7 @@ use std::sync::Arc;
 use project::Project;
 use std::sync::Arc;
 
+#[cfg(not(target_os = "freebsd"))]
 pub use livekit_client::id::TrackSid;
 pub use livekit_client::track::{RemoteAudioTrack, RemoteVideoTrack};
 
@@ -58,13 +61,18 @@ pub struct RemoteParticipant {
     pub participant_index: ParticipantIndex,
     pub muted: bool,
     pub speaking: bool,
+    #[cfg(not(target_os = "freebsd"))]
     pub video_tracks: HashMap<TrackSid, RemoteVideoTrack>,
+    #[cfg(not(target_os = "freebsd"))]
     pub audio_tracks: HashMap<TrackSid, (RemoteAudioTrack, AudioStream)>,
 }
 
 impl RemoteParticipant {
     pub fn has_video_tracks(&self) -> bool {
-        !self.video_tracks.is_empty()
+        #[cfg(not(target_os = "freebsd"))]
+        return !self.video_tracks.is_empty();
+        #[cfg(target_os = "freebsd")]
+        return false;
     }
 
     pub fn can_write(&self) -> bool {
