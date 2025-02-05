--- crates/livekit_client/src/test/participant.rs.orig	2025-02-05 13:38:44 UTC
+++ crates/livekit_client/src/test/participant.rs
@@ -8,16 +8,19 @@ pub struct LocalParticipant {
 
 #[derive(Clone, Debug)]
 pub struct LocalParticipant {
+    #[cfg(not(target_os = "freebsd"))]
     pub(super) identity: ParticipantIdentity,
     pub(super) room: Room,
 }
 
 #[derive(Clone, Debug)]
 pub struct RemoteParticipant {
+    #[cfg(not(target_os = "freebsd"))]
     pub(super) identity: ParticipantIdentity,
     pub(super) room: WeakRoom,
 }
 
+#[cfg(not(target_os = "freebsd"))]
 impl Participant {
     pub fn identity(&self) -> ParticipantIdentity {
         match self {
@@ -27,6 +30,7 @@ impl Participant {
     }
 }
 
+#[cfg(not(target_os = "freebsd"))]
 impl LocalParticipant {
     pub async fn unpublish_track(&self, track: &TrackSid) -> Result<()> {
         self.room
@@ -60,6 +64,7 @@ impl LocalParticipant {
     }
 }
 
+#[cfg(not(target_os = "freebsd"))]
 impl RemoteParticipant {
     pub fn track_publications(&self) -> HashMap<TrackSid, RemoteTrackPublication> {
         if let Some(room) = self.room.upgrade() {
