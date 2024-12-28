--- crates/call/src/cross_platform/room.rs.orig	2024-12-18 16:41:33 UTC
+++ crates/call/src/cross_platform/room.rs
@@ -1,4 +1,4 @@
-#![cfg_attr(target_os = "windows", allow(unused))]
+#![cfg_attr(any(target_os = "windows", target_os = "freebsd"), allow(unused))]
 
 use crate::{
     call_settings::CallSettings,
@@ -17,7 +17,7 @@ use language::LanguageRegistry;
     AppContext, AsyncAppContext, Context, EventEmitter, Model, ModelContext, Task, WeakModel,
 };
 use language::LanguageRegistry;
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 use livekit::{
     capture_local_audio_track, capture_local_video_track,
     id::ParticipantIdentity,
@@ -27,7 +27,7 @@ use livekit::{
     track::{TrackKind, TrackSource},
     RoomEvent, RoomOptions,
 };
-#[cfg(target_os = "windows")]
+#[cfg(any(target_os = "windows", target_os = "freebsd"))]
 use livekit::{publication::LocalTrackPublication, RoomEvent};
 use livekit_client as livekit;
 use postage::{sink::Sink, stream::Stream, watch};
@@ -106,7 +106,7 @@ impl Room {
         !self.shared_projects.is_empty()
     }
 
-    #[cfg(all(any(test, feature = "test-support"), not(target_os = "windows")))]
+    #[cfg(all(any(test, feature = "test-support"), not(any(target_os = "windows", target_os = "freebsd"))))]
     pub fn is_connected(&self) -> bool {
         if let Some(live_kit) = self.live_kit.as_ref() {
             live_kit.room.connection_state() == livekit::ConnectionState::Connected
@@ -671,7 +671,7 @@ impl Room {
         }
     }
 
-    #[cfg(target_os = "windows")]
+    #[cfg(any(target_os = "windows", target_os = "freebsd"))]
     fn start_room_connection(
         &self,
         mut room: proto::Room,
@@ -680,7 +680,7 @@ impl Room {
         Task::ready(())
     }
 
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     fn start_room_connection(
         &self,
         mut room: proto::Room,
@@ -837,7 +837,7 @@ impl Room {
                                     muted: true,
                                     speaking: false,
                                     video_tracks: Default::default(),
-                                    #[cfg(not(target_os = "windows"))]
+                                    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
                                     audio_tracks: Default::default(),
                                 },
                             );
@@ -944,7 +944,7 @@ impl Room {
         );
 
         match event {
-            #[cfg(not(target_os = "windows"))]
+            #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
             RoomEvent::TrackSubscribed {
                 track,
                 participant,
@@ -979,7 +979,7 @@ impl Room {
                 }
             }
 
-            #[cfg(not(target_os = "windows"))]
+            #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
             RoomEvent::TrackUnsubscribed {
                 track, participant, ..
             } => {
@@ -1007,7 +1007,7 @@ impl Room {
                 }
             }
 
-            #[cfg(not(target_os = "windows"))]
+            #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
             RoomEvent::ActiveSpeakersChanged { speakers } => {
                 let mut speaker_ids = speakers
                     .into_iter()
@@ -1024,7 +1024,7 @@ impl Room {
                 }
             }
 
-            #[cfg(not(target_os = "windows"))]
+            #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
             RoomEvent::TrackMuted {
                 participant,
                 publication,
@@ -1049,7 +1049,7 @@ impl Room {
                 }
             }
 
-            #[cfg(not(target_os = "windows"))]
+            #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
             RoomEvent::LocalTrackUnpublished { publication, .. } => {
                 log::info!("unpublished track {}", publication.sid());
                 if let Some(room) = &mut self.live_kit {
@@ -1072,12 +1072,12 @@ impl Room {
                 }
             }
 
-            #[cfg(not(target_os = "windows"))]
+            #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
             RoomEvent::LocalTrackPublished { publication, .. } => {
                 log::info!("published track {:?}", publication.sid());
             }
 
-            #[cfg(not(target_os = "windows"))]
+            #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
             RoomEvent::Disconnected { reason } => {
                 log::info!("disconnected from room: {reason:?}");
                 self.leave(cx).detach_and_log_err(cx);
@@ -1304,7 +1304,7 @@ impl Room {
         #[cfg(not(any(test, feature = "test-support")))]
         {
             use feature_flags::FeatureFlagAppExt as _;
-            if cfg!(target_os = "windows") || (cfg!(target_os = "linux") && !_cx.is_staff()) {
+            if cfg!(target_os = "windows") || (cfg!(any(target_os = "linux", target_os = "freebsd")) && !_cx.is_staff()) {
                 return false;
             }
         }
@@ -1323,12 +1323,12 @@ impl Room {
         }
     }
 
-    #[cfg(target_os = "windows")]
+    #[cfg(any(target_os = "windows", target_os = "freebsd"))]
     pub fn share_microphone(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
         Task::ready(Err(anyhow!("Windows is not supported yet")))
     }
 
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     #[track_caller]
     pub fn share_microphone(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
         if self.status.is_offline() {
@@ -1406,12 +1406,12 @@ impl Room {
         })
     }
 
-    #[cfg(target_os = "windows")]
+    #[cfg(any(target_os = "windows", target_os = "freebsd"))]
     pub fn share_screen(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
         Task::ready(Err(anyhow!("Windows is not supported yet")))
     }
 
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     pub fn share_screen(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
         if self.status.is_offline() {
             return Task::ready(Err(anyhow!("room is offline")));
@@ -1559,7 +1559,7 @@ impl Room {
             LocalTrack::Published {
                 track_publication, ..
             } => {
-                #[cfg(not(target_os = "windows"))]
+                #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
                 {
                     let local_participant = live_kit.room.local_participant();
                     let sid = track_publication.sid();
@@ -1575,7 +1575,7 @@ impl Room {
     }
 
     fn set_deafened(&mut self, deafened: bool, cx: &mut ModelContext<Self>) -> Option<()> {
-        #[cfg(not(target_os = "windows"))]
+        #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
         {
             let live_kit = self.live_kit.as_mut()?;
             cx.notify();
@@ -1617,7 +1617,7 @@ impl Room {
             LocalTrack::Published {
                 track_publication, ..
             } => {
-                #[cfg(not(target_os = "windows"))]
+                #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
                 {
                     if should_mute {
                         track_publication.mute()
@@ -1631,14 +1631,14 @@ impl Room {
     }
 }
 
-#[cfg(target_os = "windows")]
+#[cfg(any(target_os = "windows", target_os = "freebsd"))]
 fn spawn_room_connection(
     livekit_connection_info: Option<proto::LiveKitConnectionInfo>,
     cx: &mut ModelContext<'_, Room>,
 ) {
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 fn spawn_room_connection(
     livekit_connection_info: Option<proto::LiveKitConnectionInfo>,
     cx: &mut ModelContext<'_, Room>,
@@ -1703,10 +1703,10 @@ impl LiveKitRoom {
 }
 
 impl LiveKitRoom {
-    #[cfg(target_os = "windows")]
+    #[cfg(any(target_os = "windows", target_os = "freebsd"))]
     fn stop_publishing(&mut self, _cx: &mut ModelContext<Room>) {}
 
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     fn stop_publishing(&mut self, cx: &mut ModelContext<Room>) {
         let mut tracks_to_unpublish = Vec::new();
         if let LocalTrack::Published {
