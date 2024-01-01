use std::{
  cell::RefCell,
  mem::swap,
  sync::{Arc, RwLock},
};

use rayon::prelude::*;

use glam::{DVec2, IVec2, IVec4};
use rand::Rng;
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
    let h = window_size.y;
    let dir = world.player.direction;
    let plane = world.plane;
    let pos = world.player.position;

    let mut draw_pixel = |x: usize, y: usize, r: u8, b: u8, g: u8, a: u8| {
      let index = y * pitch + x * 4;

      buffer[index] = r;
      buffer[index + 1] = g;
      buffer[index + 2] = b;
      buffer[index + 3] = a;
    };

    // This closure is from: https://github.com/ssloy/tinyrenderer/wiki/Lesson-1:-Bresenham%E2%80%99s-Line-Drawing-Algorithm#timings-fifth-and-final-attempt
    let mut draw_line = |_x0: i32, _y0: i32, _x1: i32, _y1: i32, r: u8, g: u8, b: u8, a: u8| {
      let mut x0 = _x0;
      let mut x1 = _x1;
      let mut y0 = _y0;
      let mut y1 = _y1;
      let mut steep = false;

      if (x0 - x1).abs() < (y0 - y1).abs() {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
        steep = true;
      }
      if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
      }

      let dx = x1 - x0;
      let dy = y1 - y0;
      let derror2 = dy.abs() * 2;
      let mut error2 = 0;
      let mut y = y0;

      for x in x0..=x1 {
        if steep {
          draw_pixel(y as usize, x as usize, r, b, g, a);
        } else {
          draw_pixel(x as usize, y as usize, r, b, g, a);
        }
        error2 += derror2;
        if error2 > dx {
          y += if y1 > y0 { 1 } else { -1 };
          error2 -= dx * 2;
        }
      }
    };

    // The original tutorial is absurdly unsafe so I fixed it up.

    for x in 0..w {
      let camera_x = 2.0 * (x as f64) / (w as f64) - 1.0;

      let ray_direction = DVec2::new(dir.x + plane.x * camera_x, 0.0);

      let mut side_dist = DVec2::new(0.0, 0.0);

      let mut map_pos = IVec2::new(pos.x.floor() as i32, pos.y.floor() as i32);

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

      let perp_wall_dist;

      let mut step = IVec2::new(0, 0);

      let mut hit = 0;

      let mut side = 0;

      if ray_direction.x < 0.0 {
        step.x = -1;
        side_dist.x = (pos.x - map_pos.x as f64) * delta_dist.x;
      } else {
        step.x = 1;
        side_dist.x = (map_pos.x as f64 + 1.0 - pos.x) * delta_dist.x
      }

      if ray_direction.y < 0.0 {
        step.y = -1;
        side_dist.y = (pos.y - map_pos.y as f64) * delta_dist.y;
      } else {
        step.y = 1;
        side_dist.y = (map_pos.y as f64 + 1.0 - pos.y) * delta_dist.y;
      }

      while hit == 0 {
        if side_dist.x < side_dist.y {
          side_dist.x += delta_dist.x;
          map_pos.x += step.x;
          side = 0;
        } else {
          side_dist.y += delta_dist.y;
          map_pos.y += step.y;
          side = 1;
        }
        // Check if we hit a wall.
        if world.map.data[map_pos.x as usize][map_pos.y as usize] > 0 {
          hit = 1;
        }
      }

      if side == 0 {
        perp_wall_dist = side_dist.x - delta_dist.x;
      } else {
        perp_wall_dist = side_dist.y - delta_dist.y;
      }

      let line_height = (h as f64 / perp_wall_dist).floor() as i32;

      let mut draw_start = -line_height / 2 + h / 2;
      if draw_start < 0 {
        draw_start = 0
      }
      let mut draw_end = line_height / 2 + h / 2;
      if draw_end >= h {
        draw_end = h - 1;
      }

      let mut r: u8 = 0;
      let mut g: u8 = 0;
      let mut b: u8 = 0;
      let mut a: u8 = 255;

      match world.map.data[map_pos.x as usize][map_pos.y as usize] {
        1 => {
          r = 255;
        }
        2 => {
          g = 255;
        }
        3 => {
          b = 255;
        }
        4 => {
          r = 255;
          g = 255;
        }
        _ => (),
      }

      if side == 1 {
        r /= 2;
        g /= 2;
        b /= 2;
      }

      // draw_line(x, draw_start, x, draw_end, r, g, b, a);
    }

    (0..window_size.y).for_each(|y| {
      let mut random = rand::thread_rng();
      let mut cool = || -> u8 { random.gen_range(0..255) as u8 };
      for x in 0..window_size.x {
        draw_pixel(x as usize, y as usize, cool(), cool(), cool(), cool());
      }
    });

    // || {

    // }
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
