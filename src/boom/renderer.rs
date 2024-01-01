use super::win::Win;

pub struct Renderer {}

///
/// Renderer simply encapsulates the logic for drawing the game.
///
impl Renderer {
  pub fn new() -> Self {
    Renderer {}
  }

  ///
  /// Handles all logic for drawing things to the Window's framebuffer.
  ///
  pub fn draw(&self, window: &mut Win) {
    window.change_title("I'm drawing yay");
  }
}
