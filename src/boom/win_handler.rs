#[cfg(feature = "raw-window-handle")]
use std::ops::Deref;

use glam::IVec2;
use sdl2::{
  event::{self, EventPollIterator},
  pixels::PixelFormatEnum,
  rect::Rect,
  render::{Canvas, Texture},
  video::Window,
  Sdl, VideoSubsystem,
};

///
/// Win encapsulates the Window components to clean up the
/// external implementation and allow more flexible execution
/// during runtime.
///
pub struct WinHandler {
  sdl_context: Option<Sdl>,
  video_subsystem: Option<VideoSubsystem>,
  pub canvas: Option<Canvas<Window>>,
  pub quit_received: bool,
  pub window_size: IVec2,
}

impl WinHandler {
  pub fn new() -> Self {
    let mut new_window = WinHandler {
      sdl_context: None,
      video_subsystem: None,
      canvas: None,
      quit_received: false,
      window_size: IVec2::new(512, 512),
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
      .window(
        "test1234",
        new_window.window_size.x as u32,
        new_window.window_size.y as u32,
      )
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

    new_window
  }

  ///
  /// Changes the game window title to whatever you want.
  ///
  pub fn change_title(&mut self, new_title: &str) {
    self
      .canvas
      .as_mut()
      .unwrap()
      .window_mut()
      .set_title(new_title)
      .unwrap();
  }

  ///
  /// Draws a texture straight into the canvas.
  ///
  pub fn draw(&mut self, texture: &Texture) {
    self
      .canvas
      .as_mut()
      .unwrap()
      .copy(
        texture,
        None,
        Rect::new(0, 0, self.window_size.x as u32, self.window_size.y as u32),
      )
      .unwrap();

    self.canvas.as_mut().unwrap().present();
  }

  ///
  /// Consider this glfw's glfwPollEvents but not.
  ///
  pub fn poll(&mut self) {
    let mut event_pump = self.sdl_context.as_ref().unwrap().event_pump().unwrap();

    for event in event_pump.poll_iter() {
      match event {
        event::Event::Quit { timestamp } => {
          self.quit_received = true;
        }
        event::Event::Window {
          timestamp,
          window_id,
          win_event,
        } => match win_event {
          event::WindowEvent::Resized(x, y) => {
            self.window_size.x = x;
            self.window_size.y = y;
          }
          _ => (),
        },
        _ => (),
      }
    }
  }
}
