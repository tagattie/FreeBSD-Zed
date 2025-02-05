--- crates/livekit_client/src/test/webrtc.rs.orig	2025-02-05 09:54:57 UTC
+++ crates/livekit_client/src/test/webrtc.rs
@@ -1,5 +1,6 @@ use futures::Stream;
 use super::track::{RtcAudioTrack, RtcVideoTrack};
 use futures::Stream;
+#[cfg(not(target_os = "freebsd"))]
 use livekit::webrtc as real;
 use std::{
     pin::Pin,
@@ -11,6 +12,7 @@ pub mod video_stream {
 
     pub mod native {
         use super::*;
+        #[cfg(not(target_os = "freebsd"))]
         use real::video_frame::BoxVideoFrame;
 
         pub struct NativeVideoStream {
@@ -38,6 +40,7 @@ pub mod audio_stream {
 
     pub mod native {
         use super::*;
+        #[cfg(not(target_os = "freebsd"))]
         use real::audio_frame::AudioFrame;
 
         pub struct NativeAudioStream {
@@ -63,12 +66,14 @@ pub mod audio_source {
 pub mod audio_source {
     use super::*;
 
+    #[cfg(not(target_os = "freebsd"))]
     pub use real::audio_source::AudioSourceOptions;
 
     pub mod native {
         use std::sync::Arc;
 
         use super::*;
+        #[cfg(not(target_os = "freebsd"))]
         use real::{audio_frame::AudioFrame, RtcError};
 
         #[derive(Clone)]
@@ -103,17 +108,21 @@ pub mod audio_source {
     }
 }
 
+#[cfg(not(target_os = "freebsd"))]
 pub use livekit::webrtc::audio_frame;
+#[cfg(not(target_os = "freebsd"))]
 pub use livekit::webrtc::video_frame;
 
 pub mod video_source {
     use super::*;
+    #[cfg(not(target_os = "freebsd"))]
     pub use real::video_source::VideoResolution;
 
     pub struct RTCVideoSource;
 
     pub mod native {
         use super::*;
+        #[cfg(not(target_os = "freebsd"))]
         use real::video_frame::{VideoBuffer, VideoFrame};
 
         #[derive(Clone)]
