use glam::{DVec2, IVec2};
use sdl2::{
  event::{self},
  keyboard::Keycode,
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
  mouse_captured: bool,
  pub mouse_delta: DVec2,
  mouse_sensitivity: f64,
  pub forward_down: bool,
  pub backward_down: bool,
  pub left_down: bool,
  pub right_down: bool,
}

impl WinHandler {
  pub fn new() -> Self {
    let mut new_window = WinHandler {
      sdl_context: None,
      video_subsystem: None,
      canvas: None,
      quit_received: false,
      window_size: IVec2::new(512, 512),
      mouse_captured: false,
      mouse_delta: DVec2::new(0.0, 0.0),
      mouse_sensitivity: 10.0,
      forward_down: false,
      backward_down: false,
      left_down: false,
      right_down: false,
    };

    // I'm doing this a bit differently than I usually do.
    // Since I've never used SDL2 before, I'm just going to assume
    // literally any of this can fail randomly so it's handled as so.

    // sdl2::hint::set("SDL_VIDEO_EXTERNAL_CONTEXT", "1");

    new_window.sdl_context = Some(sdl2::init().unwrap());

    let monitor = new_window
      .sdl_context
      .as_ref()
      .unwrap()
      .video()
      .unwrap()
      .display_mode(0, 0)
      .unwrap();

    new_window.video_subsystem = Some(new_window.sdl_context.as_ref().unwrap().video().unwrap());

    let window = new_window
      .video_subsystem
      .as_ref()
      .unwrap()
      .window(
        "boom",
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
      .sdl_context
      .as_ref()
      .unwrap()
      .mouse()
      .capture(true);

    new_window
  }

  fn toggle_mouse_capture(&mut self) {
    self.mouse_captured = !self.mouse_captured;

    self
      .sdl_context
      .as_ref()
      .unwrap()
      .mouse()
      .show_cursor(!self.mouse_captured);

    self
      .sdl_context
      .as_ref()
      .unwrap()
      .mouse()
      .set_relative_mouse_mode(self.mouse_captured);
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
  /// Also shows it immediately.
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
    self.mouse_delta.x = 0.0;
    self.mouse_delta.y = 0.0;
    // self.forward_down = false;
    // self.backward_down = false;
    // self.left_down = false;
    // self.right_down = false;

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
            println!("window resized | {} | {} |", x, y);
            self.window_size.x = x;
            self.window_size.y = y;
          }
          _ => (),
        },

        event::Event::MouseMotion {
          timestamp,
          window_id,
          which,
          mousestate,
          x,
          y,
          xrel,
          yrel,
        } => {
          if self.mouse_captured {
            // Brings the sensitivity into a more sensitive range
            self.mouse_delta.x = (xrel as f64 * self.mouse_sensitivity) / 1000.0;
            self.mouse_delta.y = (yrel as f64 * self.mouse_sensitivity) / 1000.0;
          }
        }

        event::Event::KeyDown {
          timestamp,
          window_id,
          keycode,
          scancode,
          keymod,
          repeat,
        } => match keycode {
          Some(key) => match key {
            Keycode::E => self.toggle_mouse_capture(),
            Keycode::W => self.forward_down = true,
            Keycode::S => self.backward_down = true,
            Keycode::A => self.left_down = true,
            Keycode::D => self.right_down = true,

            Keycode::Escape => self.quit_received = true,
            _ => (),
          },
          None => (),
        },

        event::Event::KeyUp {
          timestamp,
          window_id,
          keycode,
          scancode,
          keymod,
          repeat,
        } => match keycode {
          Some(key) => match key {
            Keycode::W => self.forward_down = false,
            Keycode::S => self.backward_down = false,
            Keycode::A => self.left_down = false,
            Keycode::D => self.right_down = false,
            _ => (),
          },
          None => (),
        },
        _ => (),
      }
    }
  }
}
