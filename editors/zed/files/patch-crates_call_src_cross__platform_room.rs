--- crates/call/src/cross_platform/room.rs.orig	2025-02-05 13:47:42 UTC
+++ crates/call/src/cross_platform/room.rs
@@ -1,3 +1,5 @@
+#![cfg_attr(target_os = "freebsd", allow(unused))]
+
 use crate::{
     call_settings::CallSettings,
     participant::{LocalParticipant, ParticipantLocation, RemoteParticipant},
@@ -15,6 +17,7 @@ use language::LanguageRegistry;
     AppContext, AsyncAppContext, Context, EventEmitter, Model, ModelContext, Task, WeakModel,
 };
 use language::LanguageRegistry;
+#[cfg(not(target_os = "freebsd"))]
 use livekit::{
     capture_local_audio_track, capture_local_video_track,
     id::ParticipantIdentity,
@@ -24,6 +27,8 @@ use livekit::{
     track::{TrackKind, TrackSource},
     RoomEvent, RoomOptions,
 };
+#[cfg(target_os = "freebsd")]
+use livekit::{publication::LocalTrackPublication, RoomEvent};
 use livekit_client as livekit;
 use postage::{sink::Sink, stream::Stream, watch};
 use project::Project;
@@ -101,7 +106,7 @@ impl Room {
         !self.shared_projects.is_empty()
     }
 
-    #[cfg(any(test, feature = "test-support"))]
+    #[cfg(all(any(test, feature = "test-support"), not(target_os = "freebsd")))]
     pub fn is_connected(&self) -> bool {
         if let Some(live_kit) = self.live_kit.as_ref() {
             live_kit.room.connection_state() == livekit::ConnectionState::Connected
@@ -666,6 +671,7 @@ impl Room {
         }
     }
 
+    #[cfg(not(target_os = "freebsd"))]
     fn start_room_connection(
         &self,
         mut room: proto::Room,
@@ -916,6 +922,15 @@ impl Room {
         })
     }
 
+    #[cfg(target_os = "freebsd")]
+    fn start_room_connection(
+        &self,
+        mut room: proto::Room,
+        cx: &mut ModelContext<Self>,
+    ) -> Task<()> {
+        Task::ready(())
+    }
+
     fn livekit_room_updated(
         &mut self,
         event: RoomEvent,
@@ -928,6 +943,7 @@ impl Room {
         );
 
         match event {
+            #[cfg(not(target_os = "freebsd"))]
             RoomEvent::TrackSubscribed {
                 track,
                 participant,
@@ -962,6 +978,7 @@ impl Room {
                 }
             }
 
+            #[cfg(not(target_os = "freebsd"))]
             RoomEvent::TrackUnsubscribed {
                 track, participant, ..
             } => {
@@ -989,6 +1006,7 @@ impl Room {
                 }
             }
 
+            #[cfg(not(target_os = "freebsd"))]
             RoomEvent::ActiveSpeakersChanged { speakers } => {
                 let mut speaker_ids = speakers
                     .into_iter()
@@ -1005,6 +1023,7 @@ impl Room {
                 }
             }
 
+            #[cfg(not(target_os = "freebsd"))]
             RoomEvent::TrackMuted {
                 participant,
                 publication,
@@ -1029,6 +1048,7 @@ impl Room {
                 }
             }
 
+            #[cfg(not(target_os = "freebsd"))]
             RoomEvent::LocalTrackUnpublished { publication, .. } => {
                 log::info!("unpublished track {}", publication.sid());
                 if let Some(room) = &mut self.live_kit {
@@ -1051,10 +1071,12 @@ impl Room {
                 }
             }
 
+            #[cfg(not(target_os = "freebsd"))]
             RoomEvent::LocalTrackPublished { publication, .. } => {
                 log::info!("published track {:?}", publication.sid());
             }
 
+            #[cfg(not(target_os = "freebsd"))]
             RoomEvent::Disconnected { reason } => {
                 log::info!("disconnected from room: {reason:?}");
                 self.leave(cx).detach_and_log_err(cx);
@@ -1298,6 +1320,7 @@ impl Room {
         }
     }
 
+    #[cfg(not(target_os = "freebsd"))]
     #[track_caller]
     pub fn share_microphone(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
         if self.status.is_offline() {
@@ -1375,6 +1398,12 @@ impl Room {
         })
     }
 
+    #[cfg(target_os = "freebsd")]
+    pub fn share_microphone(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
+        Task::ready(Err(anyhow!("FreeBSD is not supported yet")))
+    }
+
+    #[cfg(not(target_os = "freebsd"))]
     pub fn share_screen(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
         if self.status.is_offline() {
             return Task::ready(Err(anyhow!("room is offline")));
@@ -1460,6 +1489,11 @@ impl Room {
         })
     }
 
+    #[cfg(target_os = "freebsd")]
+    pub fn share_screen(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
+        Task::ready(Err(anyhow!("FreeBSD is not supported yet")))
+    }
+
     pub fn toggle_mute(&mut self, cx: &mut ModelContext<Self>) {
         if let Some(live_kit) = self.live_kit.as_mut() {
             // When unmuting, undeafen if the user was deafened before.
@@ -1522,13 +1556,15 @@ impl Room {
             LocalTrack::Published {
                 track_publication, ..
             } => {
-                let local_participant = live_kit.room.local_participant();
-                let sid = track_publication.sid();
-                cx.background_executor()
-                    .spawn(async move { local_participant.unpublish_track(&sid).await })
-                    .detach_and_log_err(cx);
-                cx.notify();
-
+                #[cfg(not(target_os = "freebsd"))]
+                {
+                    let local_participant = live_kit.room.local_participant();
+                    let sid = track_publication.sid();
+                    cx.background_executor()
+                        .spawn(async move { local_participant.unpublish_track(&sid).await })
+                        .detach_and_log_err(cx);
+                    cx.notify();
+                }
                 Audio::play_sound(Sound::StopScreenshare, cx);
                 Ok(())
             }
@@ -1536,12 +1572,15 @@ impl Room {
     }
 
     fn set_deafened(&mut self, deafened: bool, cx: &mut ModelContext<Self>) -> Option<()> {
-        let live_kit = self.live_kit.as_mut()?;
-        cx.notify();
-        for (_, participant) in live_kit.room.remote_participants() {
-            for (_, publication) in participant.track_publications() {
-                if publication.kind() == TrackKind::Audio {
-                    publication.set_enabled(!deafened);
+        #[cfg(not(target_os = "freebsd"))]
+        {
+            let live_kit = self.live_kit.as_mut()?;
+            cx.notify();
+            for (_, participant) in live_kit.room.remote_participants() {
+                for (_, publication) in participant.track_publications() {
+                    if publication.kind() == TrackKind::Audio {
+                        publication.set_enabled(!deafened);
+                    }
                 }
             }
         }
@@ -1575,18 +1614,21 @@ impl Room {
             LocalTrack::Published {
                 track_publication, ..
             } => {
-                if should_mute {
-                    track_publication.mute()
-                } else {
-                    track_publication.unmute()
+                #[cfg(not(target_os = "freebsd"))]
+                {
+                    if should_mute {
+                        track_publication.mute()
+                    } else {
+                        track_publication.unmute()
+                    }
                 }
-
                 None
             }
         }
     }
 }
 
+#[cfg(not(target_os = "freebsd"))]
 fn spawn_room_connection(
     livekit_connection_info: Option<proto::LiveKitConnectionInfo>,
     cx: &mut ModelContext<'_, Room>,
@@ -1638,6 +1680,13 @@ fn spawn_room_connection(
     }
 }
 
+#[cfg(target_os = "freebsd")]
+fn spawn_room_connection(
+    livekit_connection_info: Option<proto::LiveKitConnectionInfo>,
+    cx: &mut ModelContext<'_, Room>,
+) {
+}
+
 struct LiveKitRoom {
     room: Arc<livekit::Room>,
     screen_track: LocalTrack,
@@ -1651,6 +1700,7 @@ impl LiveKitRoom {
 }
 
 impl LiveKitRoom {
+    #[cfg(not(target_os = "freebsd"))]
     fn stop_publishing(&mut self, cx: &mut ModelContext<Room>) {
         let mut tracks_to_unpublish = Vec::new();
         if let LocalTrack::Published {
@@ -1678,6 +1728,9 @@ impl LiveKitRoom {
             })
             .detach();
     }
+
+    #[cfg(target_os = "freebsd")]
+    fn stop_publishing(&mut self, _cx: &mut ModelContext<Room>) {}
 }
 
 enum LocalTrack {
