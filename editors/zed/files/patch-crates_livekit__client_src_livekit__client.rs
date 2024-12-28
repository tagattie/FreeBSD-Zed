--- crates/livekit_client/src/livekit_client.rs.orig	2024-12-28 12:12:14 UTC
+++ crates/livekit_client/src/livekit_client.rs
@@ -1,7 +1,7 @@
-#![cfg_attr(target_os = "windows", allow(unused))]
+#![cfg_attr(any(target_os = "windows", target_os = "freebsd"), allow(unused))]
 
 mod remote_video_track_view;
-#[cfg(any(test, feature = "test-support", target_os = "windows"))]
+#[cfg(any(test, feature = "test-support", target_os = "windows", target_os = "freebsd"))]
 pub mod test;
 
 use anyhow::{anyhow, Context as _, Result};
@@ -13,7 +13,7 @@ use util::{debug_panic, ResultExt as _};
 use parking_lot::Mutex;
 use std::{borrow::Cow, collections::VecDeque, future::Future, pin::Pin, sync::Arc, thread};
 use util::{debug_panic, ResultExt as _};
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 use webrtc::{
     audio_frame::AudioFrame,
     audio_source::{native::NativeAudioSource, AudioSourceOptions, RtcAudioSource},
@@ -23,13 +23,13 @@ use webrtc::{
     video_stream::native::NativeVideoStream,
 };
 
-#[cfg(all(not(any(test, feature = "test-support")), not(target_os = "windows")))]
+#[cfg(all(not(any(test, feature = "test-support")), not(any(target_os = "windows", target_os = "freebsd"))))]
 use livekit::track::RemoteAudioTrack;
-#[cfg(all(not(any(test, feature = "test-support")), not(target_os = "windows")))]
+#[cfg(all(not(any(test, feature = "test-support")), not(any(target_os = "windows", target_os = "freebsd"))))]
 pub use livekit::*;
-#[cfg(any(test, feature = "test-support", target_os = "windows"))]
+#[cfg(any(test, feature = "test-support", target_os = "windows", target_os = "freebsd"))]
 use test::track::RemoteAudioTrack;
-#[cfg(any(test, feature = "test-support", target_os = "windows"))]
+#[cfg(any(test, feature = "test-support", target_os = "windows", target_os = "freebsd"))]
 pub use test::*;
 
 pub use remote_video_track_view::{RemoteVideoTrackView, RemoteVideoTrackViewEvent};
@@ -46,7 +46,7 @@ struct Dispatcher(Arc<dyn gpui::PlatformDispatcher>);
 
 struct Dispatcher(Arc<dyn gpui::PlatformDispatcher>);
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 impl livekit::dispatcher::Dispatcher for Dispatcher {
     fn dispatch(&self, runnable: livekit::dispatcher::Runnable) {
         self.0.dispatch(runnable, None);
@@ -68,7 +68,7 @@ fn http_2_status(status: http_client::http::StatusCode
         .expect("valid status code to status code conversion")
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 impl livekit::dispatcher::HttpClient for HttpClientAdapter {
     fn get(
         &self,
@@ -123,14 +123,14 @@ impl livekit::dispatcher::HttpClient for HttpClientAda
     }
 }
 
-#[cfg(target_os = "windows")]
+#[cfg(any(target_os = "windows", target_os = "freebsd"))]
 pub fn init(
     dispatcher: Arc<dyn gpui::PlatformDispatcher>,
     http_client: Arc<dyn http_client::HttpClient>,
 ) {
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 pub fn init(
     dispatcher: Arc<dyn gpui::PlatformDispatcher>,
     http_client: Arc<dyn http_client::HttpClient>,
@@ -139,7 +139,7 @@ pub fn init(
     livekit::dispatcher::set_http_client(HttpClientAdapter(http_client));
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 pub async fn capture_local_video_track(
     capture_source: &dyn ScreenCaptureSource,
 ) -> Result<(track::LocalVideoTrack, Box<dyn ScreenCaptureStream>)> {
@@ -173,7 +173,7 @@ pub async fn capture_local_video_track(
     ))
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 pub fn capture_local_audio_track(
     background_executor: &BackgroundExecutor,
 ) -> Result<Task<(track::LocalAudioTrack, AudioStream)>> {
@@ -265,7 +265,7 @@ pub fn capture_local_audio_track(
     }))
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 pub fn play_remote_audio_track(
     track: &RemoteAudioTrack,
     background_executor: &BackgroundExecutor,
@@ -336,7 +336,7 @@ fn default_device(input: bool) -> anyhow::Result<(cpal
     Ok((device, config))
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 fn get_default_output() -> anyhow::Result<(cpal::Device, cpal::SupportedStreamConfig)> {
     let host = cpal::default_host();
     let output_device = host
@@ -346,7 +346,7 @@ fn get_default_output() -> anyhow::Result<(cpal::Devic
     Ok((output_device, output_config))
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 fn start_output_stream(
     output_config: cpal::SupportedStreamConfig,
     output_device: cpal::Device,
@@ -431,14 +431,14 @@ fn start_output_stream(
     (receive_task, thread)
 }
 
-#[cfg(target_os = "windows")]
+#[cfg(any(target_os = "windows", target_os = "freebsd"))]
 pub fn play_remote_video_track(
     track: &track::RemoteVideoTrack,
 ) -> impl Stream<Item = RemoteVideoFrame> {
     futures::stream::empty()
 }
 
-#[cfg(not(target_os = "windows"))]
+#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
 pub fn play_remote_video_track(
     track: &track::RemoteVideoTrack,
 ) -> impl Stream<Item = RemoteVideoFrame> {
@@ -466,7 +466,7 @@ pub type RemoteVideoFrame = Arc<gpui::RenderImage>;
 #[cfg(not(target_os = "macos"))]
 pub type RemoteVideoFrame = Arc<gpui::RenderImage>;
 
-#[cfg(not(any(target_os = "macos", target_os = "windows")))]
+#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "freebsd")))]
 fn video_frame_buffer_from_webrtc(buffer: Box<dyn VideoBuffer>) -> Option<RemoteVideoFrame> {
     use gpui::RenderImage;
     use image::{Frame, RgbaImage};
@@ -517,7 +517,7 @@ fn video_frame_buffer_to_webrtc(frame: ScreenCaptureFr
     }
 }
 
-#[cfg(not(any(target_os = "macos", target_os = "windows")))]
+#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "freebsd")))]
 fn video_frame_buffer_to_webrtc(_frame: ScreenCaptureFrame) -> Option<impl AsRef<dyn VideoBuffer>> {
     None as Option<Box<dyn VideoBuffer>>
 }
