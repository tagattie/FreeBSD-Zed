--- crates/livekit_client/src/livekit_client.rs.orig	2025-02-05 09:41:06 UTC
+++ crates/livekit_client/src/livekit_client.rs
@@ -1,5 +1,7 @@
+#![cfg_attr(target_os = "freebsd", allow(unused))]
+
 mod remote_video_track_view;
-#[cfg(any(test, feature = "test-support"))]
+#[cfg(any(test, feature = "test-support", target_os = "freebsd"))]
 pub mod test;
 
 use anyhow::{anyhow, Context as _, Result};
@@ -11,6 +13,7 @@ use util::{debug_panic, ResultExt as _};
 use parking_lot::Mutex;
 use std::{borrow::Cow, collections::VecDeque, future::Future, pin::Pin, sync::Arc, thread};
 use util::{debug_panic, ResultExt as _};
+#[cfg(not(target_os = "freebsd"))]
 use webrtc::{
     audio_frame::AudioFrame,
     audio_source::{native::NativeAudioSource, AudioSourceOptions, RtcAudioSource},
@@ -20,13 +23,13 @@ use webrtc::{
     video_stream::native::NativeVideoStream,
 };
 
-#[cfg(not(any(test, feature = "test-support")))]
+#[cfg(not(any(test, feature = "test-support", target_os = "freebsd")))]
 use livekit::track::RemoteAudioTrack;
-#[cfg(not(any(test, feature = "test-support")))]
+#[cfg(not(any(test, feature = "test-support", target_os = "freebsd")))]
 pub use livekit::*;
-#[cfg(any(test, feature = "test-support"))]
+#[cfg(any(test, feature = "test-support", target_os = "freebsd"))]
 use test::track::RemoteAudioTrack;
-#[cfg(any(test, feature = "test-support"))]
+#[cfg(any(test, feature = "test-support", target_os = "freebsd"))]
 pub use test::*;
 
 pub use remote_video_track_view::{RemoteVideoTrackView, RemoteVideoTrackViewEvent};
@@ -43,6 +46,7 @@ struct Dispatcher(Arc<dyn gpui::PlatformDispatcher>);
 
 struct Dispatcher(Arc<dyn gpui::PlatformDispatcher>);
 
+#[cfg(not(target_os = "freebsd"))]
 impl livekit::dispatcher::Dispatcher for Dispatcher {
     fn dispatch(&self, runnable: livekit::dispatcher::Runnable) {
         self.0.dispatch(runnable, None);
@@ -64,6 +68,7 @@ fn http_2_status(status: http_client::http::StatusCode
         .expect("valid status code to status code conversion")
 }
 
+#[cfg(not(target_os = "freebsd"))]
 impl livekit::dispatcher::HttpClient for HttpClientAdapter {
     fn get(
         &self,
@@ -118,6 +123,7 @@ impl livekit::dispatcher::HttpClient for HttpClientAda
     }
 }
 
+#[cfg(not(target_os = "freebsd"))]
 pub fn init(
     dispatcher: Arc<dyn gpui::PlatformDispatcher>,
     http_client: Arc<dyn http_client::HttpClient>,
@@ -126,6 +132,14 @@ pub fn init(
     livekit::dispatcher::set_http_client(HttpClientAdapter(http_client));
 }
 
+#[cfg(target_os = "freebsd")]
+pub fn init(
+    dispatcher: Arc<dyn gpui::PlatformDispatcher>,
+    http_client: Arc<dyn http_client::HttpClient>,
+) {
+}
+
+#[cfg(not(target_os = "freebsd"))]
 pub async fn capture_local_video_track(
     capture_source: &dyn ScreenCaptureSource,
 ) -> Result<(track::LocalVideoTrack, Box<dyn ScreenCaptureStream>)> {
@@ -159,6 +173,7 @@ pub async fn capture_local_video_track(
     ))
 }
 
+#[cfg(not(target_os = "freebsd"))]
 pub fn capture_local_audio_track(
     background_executor: &BackgroundExecutor,
 ) -> Result<Task<(track::LocalAudioTrack, AudioStream)>> {
@@ -169,7 +184,7 @@ pub fn capture_local_audio_track(
     let sample_rate;
     let channels;
 
-    if cfg!(any(test, feature = "test-support")) {
+    if cfg!(any(test, feature = "test-support", target_os = "freebsd")) {
         sample_rate = 2;
         channels = 1;
     } else {
@@ -250,6 +265,7 @@ pub fn capture_local_audio_track(
     }))
 }
 
+#[cfg(not(target_os = "freebsd"))]
 pub fn play_remote_audio_track(
     track: &RemoteAudioTrack,
     background_executor: &BackgroundExecutor,
@@ -257,7 +273,7 @@ pub fn play_remote_audio_track(
     let track = track.clone();
     // We track device changes in our output because Livekit has a resampler built in,
     // and it's easy to create a new native audio stream when the device changes.
-    if cfg!(any(test, feature = "test-support")) {
+    if cfg!(any(test, feature = "test-support", target_os = "freebsd")) {
         Ok(AudioStream::Output {
             _task: background_executor.spawn(async {}),
         })
@@ -299,6 +315,13 @@ pub fn play_remote_audio_track(
     }
 }
 
+#[cfg(target_os = "freebsd")]
+pub fn play_remote_video_track(
+    track: &track::RemoteVideoTrack,
+) -> impl Stream<Item = RemoteVideoFrame> {
+    futures::stream::empty()
+}
+
 fn default_device(input: bool) -> anyhow::Result<(cpal::Device, cpal::SupportedStreamConfig)> {
     let device;
     let config;
@@ -320,6 +343,7 @@ fn default_device(input: bool) -> anyhow::Result<(cpal
     Ok((device, config))
 }
 
+#[cfg(not(target_os = "freebsd"))]
 fn get_default_output() -> anyhow::Result<(cpal::Device, cpal::SupportedStreamConfig)> {
     let host = cpal::default_host();
     let output_device = host
@@ -329,6 +353,7 @@ fn get_default_output() -> anyhow::Result<(cpal::Devic
     Ok((output_device, output_config))
 }
 
+#[cfg(not(target_os = "freebsd"))]
 fn start_output_stream(
     output_config: cpal::SupportedStreamConfig,
     output_device: cpal::Device,
@@ -372,7 +397,7 @@ fn start_output_stream(
     // and we experienced a deadlock when it's created on the main thread.
     let (thread, end_on_drop_rx) = std::sync::mpsc::channel::<()>();
     thread::spawn(move || {
-        if cfg!(any(test, feature = "test-support")) {
+        if cfg!(any(test, feature = "test-support", target_os = "freebsd")) {
             // Can't play audio in tests
             return;
         }
@@ -413,6 +438,7 @@ fn start_output_stream(
     (receive_task, thread)
 }
 
+#[cfg(not(target_os = "freebsd"))]
 pub fn play_remote_video_track(
     track: &track::RemoteVideoTrack,
 ) -> impl Stream<Item = RemoteVideoFrame> {
@@ -440,7 +466,7 @@ pub type RemoteVideoFrame = Arc<gpui::RenderImage>;
 #[cfg(not(target_os = "macos"))]
 pub type RemoteVideoFrame = Arc<gpui::RenderImage>;
 
-#[cfg(not(target_os = "macos"))]
+#[cfg(not(any(target_os = "macos", target_os = "freebsd")))]
 fn video_frame_buffer_from_webrtc(buffer: Box<dyn VideoBuffer>) -> Option<RemoteVideoFrame> {
     use gpui::RenderImage;
     use image::{Frame, RgbaImage};
@@ -491,7 +517,7 @@ fn video_frame_buffer_to_webrtc(frame: ScreenCaptureFr
     }
 }
 
-#[cfg(not(target_os = "macos"))]
+#[cfg(not(any(target_os = "macos", target_os = "freebsd")))]
 fn video_frame_buffer_to_webrtc(_frame: ScreenCaptureFrame) -> Option<impl AsRef<dyn VideoBuffer>> {
     None as Option<Box<dyn VideoBuffer>>
 }
