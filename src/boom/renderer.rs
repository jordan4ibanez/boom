use glam::{DVec2, IVec2};
use sdl2::pixels::PixelFormatEnum;

use super::{win_handler::WinHandler, world::World};

pub struct Renderer {}

///
/// Translated from C/C++ into Rust with SDL2 for fun.
///
/// If you would like to read this tutorial and translate it into yet another language:
/// https://lodev.org/cgtutor/raycasting.html
///

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
  /// This creates an oddly powerful feeling with the implementation.
  ///
  fn raycast(&mut self, world: &World, window_size: &IVec2, buffer: &mut [u8], pitch: usize) {
    // These are here to help me keep my sanity translating this tutorial.
    let w = window_size.x;
    let dir = world.player.direction;
    let plane = world.plane;
    let pos = world.player.position;
    let map_pos = IVec2::new(pos.x.floor() as i32, pos.y.floor() as i32);

    // The original tutorial is absurdly unsafe so I fixed it up.

    for x in 0..w {
      let camera_x = 2.0 * (x as f64) / (w as f64) - 1.0;

      let ray_direction = DVec2::new(dir.x + plane.x * camera_x, 0.0);

      let mut side_dist = DVec2::new(0.0, 0.0);

      let delta_dist = DVec2::new(
        if ray_direction.x == 0.0 {
          1e30
        } else {
          (1.0 / ray_direction.x).abs()
        },
        if ray_direction.y == 0.0 {
          1e30
        } else {
          (1.0 / ray_direction.y).abs()
        },
      );

      let mut perp_wall_dist = 0.0;

      let mut step = IVec2::new(0, 0);

      let mut hit = 0;

      let mut side = 0;
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

/*
this used to be in raycast, here for an example.

for y in 0..window_size.y as usize {
  for x in 0..window_size.x as usize {
    let index = y * pitch + x * 4;

    buffer[index] = x as u8;
    buffer[index + 1] = y as u8;
    buffer[index + 2] = 0;
    buffer[index + 3] = 255;
  }
}
*/
