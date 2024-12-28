--- crates/livekit_client/src/test/participant.rs.orig	2024-12-28 12:27:24 UTC
+++ crates/livekit_client/src/test/participant.rs
@@ -8,19 +8,19 @@ pub struct LocalParticipant {
 
 #[derive(Clone, Debug)]
 pub struct LocalParticipant {
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     pub(super) identity: ParticipantIdentity,
     pub(super) room: Room,
 }
 
 #[derive(Clone, Debug)]
 pub struct RemoteParticipant {
-    #[cfg(not(target_os = "windows"))]
+    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
     pub(super) identity: ParticipantIdentity,
     pub(super) room: WeakRoom,
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 impl Participant {
     pub fn identity(&self) -> ParticipantIdentity {
         match self {
@@ -30,7 +30,7 @@ impl Participant {
     }
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 impl LocalParticipant {
     pub async fn unpublish_track(&self, track: &TrackSid) -> Result<()> {
         self.room
@@ -64,7 +64,7 @@ impl LocalParticipant {
     }
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 impl RemoteParticipant {
     pub fn track_publications(&self) -> HashMap<TrackSid, RemoteTrackPublication> {
         if let Some(room) = self.room.upgrade() {
