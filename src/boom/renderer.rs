use std::mem::swap;

use glam::IVec2;

use sdl2::pixels::PixelFormatEnum;

use super::{win_handler::WinHandler, world::World};

pub struct Renderer {
  texture_size: i32,
}

///
/// Translated from C/C++/TypeScript into Rust with SDL2 for fun.
///
/// If you would like to read this tutorial and translate it into yet another language:
/// https://lodev.org/cgtutor/raycasting.html
///

///
/// Renderer simply encapsulates the logic for drawing the game.
///
/// It also stores texture data. How fancy.
///
impl Renderer {
  pub fn new() -> Self {
    Renderer { texture_size: 64 }
  }

  ///
  /// The actual raycast into the world. Draws to the framebuffer.
  ///
  /// This creates an oddly powerful feeling with the implementation.
  ///
  fn raycast(&mut self, world: &World, window_size: &IVec2, buffer: &mut [u8], pitch: usize) {
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
    // These are here to help me keep my sanity translating this tutorial.
    let w = window_size.x;
    let h = window_size.y;
    let dir = world.player.direction;
    let dirX = dir.x;
    let dirY = dir.y;
    let plane = world.plane;
    let planeX = plane.x;
    let planeY = plane.y;
    let pos = world.player.position;
    let posX = pos.x;
    let posY = pos.y;
    let worldMap = world.map.data;

    // println!("plane: {:?}", plane);

    for x in 0..w {
      //calculate ray position and direction
      let cameraX: f64 = 2.0 * (x as f64) / (w as f64) - 1.0; //x-coordinate in camera space
      let rayDirX: f64 = dirX + planeX * cameraX;
      let rayDirY = dirY + planeY * cameraX;
      //which box of the map we're in
      let mut mapX: i32 = (posX) as i32;
      let mut mapY: i32 = (posY) as i32;

      //length of ray from current position to next x or y-side
      let mut sideDistX: f64;
      let mut sideDistY: f64;

      //length of ray from one x or y-side to next x or y-side
      //these are derived as:
      //deltaDistX = sqrt(1 + (rayDirY * rayDirY) / (rayDirX * rayDirX))
      //deltaDistY = sqrt(1 + (rayDirX * rayDirX) / (rayDirY * rayDirY))
      //which can be simplified to abs(|rayDir| / rayDirX) and abs(|rayDir| / rayDirY)
      //where |rayDir| is the length of the vector (rayDirX, rayDirY). Its length,
      //unlike (dirX, dirY) is not 1, however this does not matter, only the
      //ratio between deltaDistX and deltaDistY matters, due to the way the DDA
      //stepping further below works. So the values can be computed as below.
      // Division through zero is prevented, even though technically that's not
      // needed in C++ with IEEE 754 floating point values.
      let deltaDistX: f64 = if (rayDirX == 0.0) {
        1e30
      } else {
        (1.0 / rayDirX).abs()
      };
      let deltaDistY: f64 = if (rayDirY == 0.0) {
        1e30
      } else {
        (1.0 / rayDirY).abs()
      };

      let perpWallDist: f64;

      //what direction to step in x or y-direction (either +1 or -1)
      let stepX: i32;
      let stepY: i32;

      let mut hit: i32 = 0; //was there a wall hit?
      let mut side: i32 = 0; //was a NS or a EW wall hit?
                             //calculate step and initial sideDist
      if (rayDirX < 0.0) {
        stepX = -1;
        sideDistX = (posX - (mapX as f64)) * deltaDistX;
      } else {
        stepX = 1;
        sideDistX = ((mapX as f64) + 1.0 - posX) * deltaDistX;
      }
      if (rayDirY < 0.0) {
        stepY = -1;
        sideDistY = (posY - (mapY as f64)) * deltaDistY;
      } else {
        stepY = 1;
        sideDistY = ((mapY as f64) + 1.0 - posY) * deltaDistY;
      }
      //perform DDA
      while (hit == 0) {
        //jump to next map square, either in x-direction, or in y-direction
        if (sideDistX < sideDistY) {
          sideDistX += deltaDistX;
          mapX += stepX;
          side = 0;
        } else {
          sideDistY += deltaDistY;
          mapY += stepY;
          side = 1;
        }
        //Check if ray has hit a wall
        if (worldMap[mapX as usize][mapY as usize] > 0) {
          hit = 1
        };
      }
      //Calculate distance projected on camera direction. This is the shortest distance from the point where the wall is
      //hit to the camera plane. Euclidean to center camera point would give fisheye effect!
      //This can be computed as (mapX - posX + (1 - stepX) / 2) / rayDirX for side == 0, or same formula with Y
      //for size == 1, but can be simplified to the code below thanks to how sideDist and deltaDist are computed:
      //because they were left scaled to |rayDir|. sideDist is the entire length of the ray above after the multiple
      //steps, but we subtract deltaDist once because one step more into the wall was taken above.
      if side == 0 {
        perpWallDist = (sideDistX - deltaDistX)
      } else {
        perpWallDist = (sideDistY - deltaDistY)
      };

      //Calculate height of line to draw on screen
      let lineHeight: i32 = ((h as f64) / perpWallDist) as i32;

      //calculate lowest and highest pixel to fill in current stripe
      let mut drawStart: i32 = -lineHeight / 2 + h / 2;
      if (drawStart < 0) {
        drawStart = 0
      };
      let mut drawEnd: i32 = lineHeight / 2 + h / 2;
      if (drawEnd >= h) {
        drawEnd = h - 1
      };

      //choose wall color
      let mut r: u8 = 0;
      let mut g: u8 = 0;
      let mut b: u8 = 0;
      let mut a: u8 = 0;
      match (worldMap[mapX as usize][mapY as usize]) {
        1 => r = 255, //red
        2 => g = 255, //green
        3 => b = 255, //blue
        4 => {
          r = 255;
          g = 255;
          b = 255;
        } //white
        // default: color = RGB_Yellow; break; //yellow
        _ => {
          r = 255;
          g = 255;
        }
      }

      //give x and y sides different brightness
      if (side == 1) {
        r /= 2;
        g /= 2;
        b /= 2;
      }

      //draw the pixels of the stripe as a vertical line
      draw_line(x, drawStart, x, drawEnd, r, g, b, a);
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
