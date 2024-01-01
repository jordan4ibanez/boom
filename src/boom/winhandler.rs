#[cfg(feature = "raw-window-handle")]
use std::ops::Deref;

use sdl2::{
  event, pixels::PixelFormatEnum, rect::Rect, render::Canvas, video::Window, Sdl, VideoSubsystem,
};

///
/// Win encapsulates the Window components to clean up the
/// external implementation and allow more flexible execution
/// during runtime.
///
pub struct WinHandler {
  sdl_context: Option<Sdl>,
  video_subsystem: Option<VideoSubsystem>,
  canvas: Option<Canvas<Window>>,
}

impl WinHandler {
  pub fn new() -> Self {
    let mut new_window = WinHandler {
      sdl_context: None,
      video_subsystem: None,
      canvas: None,
    };

    // I'm doing this a bit differently than I usually do.
    // Since I've never used SDL2 before, I'm just going to assume
    // literally any of this can fail randomly so it's handled as so.

    sdl2::hint::set("SDL_VIDEO_EXTERNAL_CONTEXT", "1");

    new_window.sdl_context = Some(sdl2::init().unwrap());

    new_window.video_subsystem = Some(new_window.sdl_context.as_ref().unwrap().video().unwrap());

    let window = new_window
      .video_subsystem
      .as_ref()
      .unwrap()
      .window("test1234", 549, 12345)
      .resizable()
      .position_centered()
      .build()
      .map_err(|e| panic!("{}", e))
      .unwrap();

    new_window.canvas = Some(
      window
        .into_canvas()
        .build()
        .map_err(|e| panic!("{}", e))
        .unwrap(),
    );

    // And then finally, we create the surface to draw on.
    let texture_creator = new_window.canvas.as_ref().unwrap().texture_creator();
    let mut surface = texture_creator
      .create_texture_streaming(PixelFormatEnum::RGBA8888, 256, 256)
      .map_err(|e| panic!("{}", e))
      .unwrap();

    surface
      .with_lock(None, |buffer, pitch| {
        for y in 0..256 {
          for x in 0..256 {
            let index = y * pitch + x * 4;

            buffer[index] = x as u8;
            buffer[index + 1] = y as u8;
            buffer[index + 2] = 0;
            buffer[index + 3] = 255;
          }
        }
      })
      .unwrap();

    new_window.canvas.as_mut().unwrap().clear();

    new_window
      .canvas
      .as_mut()
      .unwrap()
      .copy(&surface, None, Rect::new(0, 0, 256, 256))
      .unwrap();

    new_window.canvas.as_mut().unwrap().present();

    new_window
  }

  ///
  /// Changes the game window title to whatever you want.
  ///
  pub fn change_title(&mut self, new_title: &str) {
    // self.window.as_ref().unwrap().set_title(new_title);
  }

  ///
  /// Consider this glfw's glfwPollEvents but not.
  ///
  pub fn poll(&self) {
    let mut event_pump = self.sdl_context.as_ref().unwrap().event_pump().unwrap();

    for event in event_pump.poll_iter() {
      match event  {
        event::Event::Quit { timestamp } => todo!(),
        event::Event::AppTerminating { timestamp } => todo!(),
        event::Event::AppLowMemory { timestamp } => todo!(),
        event::Event::AppWillEnterBackground { timestamp } => todo!(),
        event::Event::AppDidEnterBackground { timestamp } => todo!(),
        event::Event::AppWillEnterForeground { timestamp } => todo!(),
        event::Event::AppDidEnterForeground { timestamp } => todo!(),
        event::Event::Display { timestamp, display_index, display_event } => todo!(),
        event::Event::Window { timestamp, window_id, win_event } => todo!(),
        event::Event::KeyDown { timestamp, window_id, keycode, scancode, keymod, repeat } => todo!(),
        event::Event::KeyUp { timestamp, window_id, keycode, scancode, keymod, repeat } => todo!(),
        event::Event::TextEditing { timestamp, window_id, text, start, length } => todo!(),
        event::Event::TextInput { timestamp, window_id, text } => todo!(),
        event::Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } => todo!(),
        event::Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => todo!(),
        event::Event::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x, y } => todo!(),
        event::Event::MouseWheel { timestamp, window_id, which, x, y, direction, precise_x, precise_y } => todo!(),
        event::Event::JoyAxisMotion { timestamp, which, axis_idx, value } => todo!(),
        event::Event::JoyBallMotion { timestamp, which, ball_idx, xrel, yrel } => todo!(),
        event::Event::JoyHatMotion { timestamp, which, hat_idx, state } => todo!(),
        event::Event::JoyButtonDown { timestamp, which, button_idx } => todo!(),
        event::Event::JoyButtonUp { timestamp, which, button_idx } => todo!(),
        event::Event::JoyDeviceAdded { timestamp, which } => todo!(),
        event::Event::JoyDeviceRemoved { timestamp, which } => todo!(),
        event::Event::ControllerAxisMotion { timestamp, which, axis, value } => todo!(),
        event::Event::ControllerButtonDown { timestamp, which, button } => todo!(),
        event::Event::ControllerButtonUp { timestamp, which, button } => todo!(),
        event::Event::ControllerDeviceAdded { timestamp, which } => todo!(),
        event::Event::ControllerDeviceRemoved { timestamp, which } => todo!(),
        event::Event::ControllerDeviceRemapped { timestamp, which } => todo!(),
        event::Event::ControllerTouchpadDown { timestamp, which, touchpad, finger, x, y, pressure } => todo!(),
        event::Event::ControllerTouchpadMotion { timestamp, which, touchpad, finger, x, y, pressure } => todo!(),
        event::Event::ControllerTouchpadUp { timestamp, which, touchpad, finger, x, y, pressure } => todo!(),
        event::Event::FingerDown { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => todo!(),
        event::Event::FingerUp { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => todo!(),
        event::Event::FingerMotion { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => todo!(),
        event::Event::DollarGesture { timestamp, touch_id, gesture_id, num_fingers, error, x, y } => todo!(),
        event::Event::DollarRecord { timestamp, touch_id, gesture_id, num_fingers, error, x, y } => todo!(),
        event::Event::MultiGesture { timestamp, touch_id, d_theta, d_dist, x, y, num_fingers } => todo!(),
        event::Event::ClipboardUpdate { timestamp } => todo!(),
        event::Event::DropFile { timestamp, window_id, filename } => todo!(),
        event::Event::DropText { timestamp, window_id, filename } => todo!(),
        event::Event::DropBegin { timestamp, window_id } => todo!(),
        event::Event::DropComplete { timestamp, window_id } => todo!(),
        event::Event::AudioDeviceAdded { timestamp, which, iscapture } => todo!(),
        event::Event::AudioDeviceRemoved { timestamp, which, iscapture } => todo!(),
        event::Event::RenderTargetsReset { timestamp } => todo!(),
        event::Event::RenderDeviceReset { timestamp } => todo!(),
        event::Event::User { timestamp, window_id, type_, code, data1, data2 } => todo!(),
        event::Event::Unknown { timestamp, type_ } => todo!(),
    }
    }
  }
}
