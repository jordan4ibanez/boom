use sdl2::pixels::PixelFormatEnum;

use super::win_handler::WinHandler;

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
  pub fn draw(&self, window: &mut WinHandler) {
    // We create a new frame buffer literally every frame.
    window.canvas.as_mut().unwrap().clear();

    let texture_creator = window.canvas.as_ref().unwrap().texture_creator();
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

    window.draw(&surface);
  }
}
