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
  pub quit_received: bool,
}

impl WinHandler {
  pub fn new() -> Self {
    let mut new_window = WinHandler {
      sdl_context: None,
      video_subsystem: None,
      canvas: None,
      quit_received: false,
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
  pub fn poll(&mut self) {
    let mut event_pump = self.sdl_context.as_ref().unwrap().event_pump().unwrap();

    for event in event_pump.poll_iter() {
      match event {
        event::Event::Quit { timestamp } => {
          self.quit_received = true;
        }
        _ => (),
      }
    }
  }
}
