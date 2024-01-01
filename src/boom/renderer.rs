use glam::IVec2;
use sdl2::pixels::PixelFormatEnum;

use super::{win_handler::WinHandler, world::World};

pub struct Renderer {}

///
/// Renderer simply encapsulates the logic for drawing the game.
///
impl Renderer {
  pub fn new() -> Self {
    Renderer {}
  }

  ///
  /// The actual raycast into the world. Draws to the framebuffer.
  ///
  /// This is oddly powerful feeling.
  ///
  fn raycast(&mut self, world: &World, window_size: &IVec2, buffer: &mut [u8], pitch: usize) {
    for y in 0..window_size.y as usize {
      for x in 0..window_size.x as usize {
        let index = y * pitch + x * 4;

        buffer[index] = x as u8;
        buffer[index + 1] = y as u8;
        buffer[index + 2] = 0;
        buffer[index + 3] = 255;
      }
    }
  }

  ///
  /// Handles all logic for drawing things to the Window's framebuffer.
  ///
  pub fn draw(&mut self, window: &mut WinHandler, world: &mut World) {
    // We create a new frame buffer literally every frame.
    window.canvas.as_mut().unwrap().clear();

    let window_size = &window.window_size;

    let texture_creator = window.canvas.as_ref().unwrap().texture_creator();
    let mut texture = texture_creator
      .create_texture_streaming(
        PixelFormatEnum::RGBA8888,
        window_size.x as u32,
        window_size.y as u32,
      )
      .map_err(|e| panic!("{}", e))
      .unwrap();

    // We can pass the self raycasting function straight into a write lock closure. Incredible.
    texture
      .with_lock(None, |buffer, pitch| {
        self.raycast(world, window_size, buffer, pitch);
      })
      .unwrap();

    window.draw(&texture);
  }
}
