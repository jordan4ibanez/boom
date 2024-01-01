use std::rc::Rc;

use sdl2::{Sdl, VideoSubsystem};
use softbuffer::{Context, Surface};

///
/// Win encapsulates the Window components to clean up the
/// external implementation and allow more flexible execution
/// during runtime.
///
pub struct WinHandler {
  // surface: Option<Surface<Rc<Window>, Rc<Window>>>,
  sdl_context: Option<Sdl>,
  video_subsystem: Option<VideoSubsystem>,
}

impl WinHandler {
  pub fn new() -> Self {
    let mut new_window = WinHandler {
      sdl_context: None,
      video_subsystem: None,
    };

    // I'm doing this a bit differently than I usually do.
    // Since I've never used SDL2 before, I'm just going to assume
    // literally any of this can fail randomly so it's handled as so.

    new_window.sdl_context = Some(sdl2::init().unwrap());

    new_window.video_subsystem = Some(new_window.sdl_context.as_ref().unwrap().video().unwrap());

    // The window context for the actual window.
    // new_window.context = Some(Context::new(new_window.window.clone().unwrap().clone()).unwrap());

    // And then finally, we create the surface to draw on.
    // new_window.surface = Some(
    //   Surface::new(
    //     new_window.context.as_ref().unwrap(),
    //     new_window.window.clone().unwrap().clone(),
    //   )
    //   .unwrap(),
    // );

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
  pub fn poll(&self) {}
}
