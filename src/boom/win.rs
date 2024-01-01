use std::rc::Rc;

use softbuffer::{Context, Surface};
use winit::{
  event_loop::EventLoop,
  window::{Window, WindowBuilder},
};

///
/// Win encapsulates the Window components to clean up the
/// external implementation and allow more flexible execution
/// during runtime.
///
pub struct Win {
  event_loop: EventLoop<()>,
  window: Option<Rc<Window>>,
  context: Option<Context<Rc<Window>>>,
  surface: Option<Surface<Rc<Window>, Rc<Window>>>,
}

impl Win {
  pub fn new() -> Self {
    let mut new_window = Win {
      event_loop: EventLoop::new().unwrap(),
      window: None,
      context: None,
      surface: None,
    };

    // The actual window.
    new_window.window = Some(Rc::new(
      WindowBuilder::new().build(&new_window.event_loop).unwrap(),
    ));

    // The window context for the actual window.
    new_window.context = Some(Context::new(new_window.window.clone().unwrap().clone()).unwrap());

    // And then finally, we create the surface to draw on.
    new_window.surface = Some(
      Surface::new(
        new_window.context.as_ref().unwrap(),
        new_window.window.clone().unwrap().clone(),
      )
      .unwrap(),
    );

    new_window
  }

  ///
  /// Changes the game window title to whatever you want.
  ///
  pub fn change_title(&mut self, new_title: &str) {
    self.window.as_ref().unwrap().set_title(new_title);
  }
}
