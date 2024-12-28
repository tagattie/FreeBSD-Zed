--- crates/livekit_client/src/test.rs.orig	2024-12-28 12:19:08 UTC
+++ crates/livekit_client/src/test.rs
@@ -2,17 +2,17 @@ pub mod track;
 pub mod publication;
 pub mod track;
 
-#[cfg(not(windows))]
+#[cfg(not(any(windows, target_os = "freebsd")))]
 pub mod webrtc;
 
-#[cfg(not(windows))]
+#[cfg(not(any(windows, target_os = "freebsd")))]
 use self::id::*;
 use self::{participant::*, publication::*, track::*};
 use anyhow::{anyhow, Context, Result};
 use async_trait::async_trait;
 use collections::{btree_map::Entry as BTreeEntry, hash_map::Entry, BTreeMap, HashMap, HashSet};
 use gpui::BackgroundExecutor;
-#[cfg(not(windows))]
+#[cfg(not(any(windows, target_os = "freebsd")))]
 use livekit::options::TrackPublishOptions;
 use livekit_server::{proto, token};
 use parking_lot::Mutex;
@@ -22,7 +22,7 @@ use std::sync::{
     Arc, Weak,
 };
 
-#[cfg(not(windows))]
+#[cfg(not(any(windows, target_os = "freebsd")))]
 pub use livekit::{id, options, ConnectionState, DisconnectReason, RoomOptions};
 
 static SERVERS: Mutex<BTreeMap<String, Arc<TestServer>>> = Mutex::new(BTreeMap::new());
@@ -31,12 +31,12 @@ pub struct TestServer {
     pub url: String,
     pub api_key: String,
     pub secret_key: String,
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     rooms: Mutex<HashMap<String, TestServerRoom>>,
     executor: BackgroundExecutor,
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 impl TestServer {
     pub fn create(
         url: String,
@@ -534,7 +534,7 @@ impl TestServer {
     }
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 #[derive(Default, Debug)]
 struct TestServerRoom {
     client_rooms: HashMap<ParticipantIdentity, Room>,
@@ -543,7 +543,7 @@ struct TestServerRoom {
     participant_permissions: HashMap<ParticipantIdentity, proto::ParticipantPermission>,
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 #[derive(Debug)]
 struct TestServerVideoTrack {
     sid: TrackSid,
@@ -551,7 +551,7 @@ struct TestServerVideoTrack {
     // frames_rx: async_broadcast::Receiver<Frame>,
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 #[derive(Debug)]
 struct TestServerAudioTrack {
     sid: TrackSid,
@@ -590,7 +590,7 @@ pub enum RoomEvent {
     TrackSubscriptionFailed {
         participant: RemoteParticipant,
         error: String,
-        #[cfg(not(target_os = "windows"))]
+        #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
         track_sid: TrackSid,
     },
     TrackPublished {
@@ -626,12 +626,12 @@ pub enum RoomEvent {
     ActiveSpeakersChanged {
         speakers: Vec<Participant>,
     },
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     ConnectionStateChanged(ConnectionState),
     Connected {
         participants_with_tracks: Vec<(RemoteParticipant, Vec<RemoteTrackPublication>)>,
     },
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     Disconnected {
         reason: DisconnectReason,
     },
@@ -639,7 +639,7 @@ pub enum RoomEvent {
     Reconnected,
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 #[async_trait]
 impl livekit_server::api::Client for TestApiClient {
     fn url(&self) -> &str {
@@ -703,11 +703,11 @@ struct RoomState {
 struct RoomState {
     url: String,
     token: String,
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     local_identity: ParticipantIdentity,
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     connection_state: ConnectionState,
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     paused_audio_tracks: HashSet<TrackSid>,
     updates_tx: mpsc::Sender<RoomEvent>,
 }
@@ -718,7 +718,7 @@ pub(crate) struct WeakRoom(Weak<Mutex<RoomState>>);
 #[derive(Clone, Debug)]
 pub(crate) struct WeakRoom(Weak<Mutex<RoomState>>);
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 impl std::fmt::Debug for RoomState {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         f.debug_struct("Room")
@@ -731,7 +731,7 @@ impl std::fmt::Debug for RoomState {
     }
 }
 
-#[cfg(target_os = "windows")]
+#[cfg(any(target_os = "windows", target_os = "freebsd"))]
 impl std::fmt::Debug for RoomState {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         f.debug_struct("Room")
@@ -741,7 +741,7 @@ impl std::fmt::Debug for RoomState {
     }
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 impl Room {
     fn downgrade(&self) -> WeakRoom {
         WeakRoom(Arc::downgrade(&self.0))
@@ -803,7 +803,7 @@ impl Room {
     }
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 impl Drop for RoomState {
     fn drop(&mut self) {
         if self.connection_state == ConnectionState::Connected {
