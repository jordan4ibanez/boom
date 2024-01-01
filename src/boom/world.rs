use std::collections::HashMap;

use glam::DVec2;
use glam::IVec2;

use super::win_handler::WinHandler;

///
/// The player. It's you!
///
pub struct Player {
  alive: bool,
  pub position: DVec2,
  pub direction: DVec2,
}

impl Player {
  pub fn new() -> Self {
    Player {
      alive: true,
      position: DVec2::new(22.0, 12.0),
      direction: DVec2::new(-1.0, 0.0),
    }
  }
}

///
/// Enemies in the game. If you can call them that.
///
pub struct Mob {
  alive: bool,
  position: DVec2,
  yaw: f64,
  sprite: usize,
}
impl Mob {
  pub fn new(position: DVec2) -> Self {
    Mob {
      alive: true,
      position,
      yaw: 0.0,
      sprite: 0,
    }
  }
}

///
/// The bullet the player can shoot. Pew pew.
///
pub struct Bullet {
  position: DVec2,
  direction: DVec2,
}

impl Bullet {
  pub fn new(position: DVec2, direction: DVec2) -> Self {
    Bullet {
      position,
      direction,
    }
  }
}

///
/// Raw map data.
///
pub struct Map {
  pub min: IVec2,
  pub max: IVec2,
  pub data: [[i32; 24]; 24],
}

impl Map {
  pub fn new() -> Self {
    Map {
      min: IVec2::new(0, 0),
      max: IVec2::new(24, 24),
      data: [
        [
          8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 4, 4, 6, 4, 4, 6, 4, 6, 4, 4, 4, 6, 4,
        ],
        [
          8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
        ],
        [
          8, 0, 3, 3, 0, 0, 0, 0, 0, 8, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
        ],
        [
          8, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
        ],
        [
          8, 0, 3, 3, 0, 0, 0, 0, 0, 8, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
        ],
        [
          8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 4, 0, 0, 0, 0, 0, 6, 6, 6, 0, 6, 4, 6,
        ],
        [
          8, 8, 8, 8, 0, 8, 8, 8, 8, 8, 8, 4, 4, 4, 4, 4, 4, 6, 0, 0, 0, 0, 0, 6,
        ],
        [
          7, 7, 7, 7, 0, 7, 7, 7, 7, 0, 8, 0, 8, 0, 8, 0, 8, 4, 0, 4, 0, 6, 0, 6,
        ],
        [
          7, 7, 0, 0, 0, 0, 0, 0, 7, 8, 0, 8, 0, 8, 0, 8, 8, 6, 0, 0, 0, 0, 0, 6,
        ],
        [
          7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 6, 0, 0, 0, 0, 0, 4,
        ],
        [
          7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 6, 0, 6, 0, 6, 0, 6,
        ],
        [
          7, 7, 0, 0, 0, 0, 0, 0, 7, 8, 0, 8, 0, 8, 0, 8, 8, 6, 4, 6, 0, 6, 6, 6,
        ],
        [
          7, 7, 7, 7, 0, 7, 7, 7, 7, 8, 8, 4, 0, 6, 8, 4, 8, 3, 3, 3, 0, 3, 3, 3,
        ],
        [
          2, 2, 2, 2, 0, 2, 2, 2, 2, 4, 6, 4, 0, 0, 6, 0, 6, 3, 0, 0, 0, 0, 0, 3,
        ],
        [
          2, 2, 0, 0, 0, 0, 0, 2, 2, 4, 0, 0, 0, 0, 0, 0, 4, 3, 0, 0, 0, 0, 0, 3,
        ],
        [
          2, 0, 0, 0, 0, 0, 0, 0, 2, 4, 0, 0, 0, 0, 0, 0, 4, 3, 0, 0, 0, 0, 0, 3,
        ],
        [
          1, 0, 0, 0, 0, 0, 0, 0, 1, 4, 4, 4, 4, 4, 6, 0, 6, 3, 3, 0, 0, 0, 3, 3,
        ],
        [
          2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 1, 2, 2, 2, 6, 6, 0, 0, 5, 0, 5, 0, 5,
        ],
        [
          2, 2, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 2, 2, 0, 5, 0, 5, 0, 0, 0, 5, 5,
        ],
        [
          2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 2, 5, 0, 5, 0, 5, 0, 5, 0, 5,
        ],
        [
          1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5,
        ],
        [
          2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 2, 5, 0, 5, 0, 5, 0, 5, 0, 5,
        ],
        [
          2, 2, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 2, 2, 0, 5, 0, 5, 0, 0, 0, 5, 5,
        ],
        [
          2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        ],
      ],
    }
  }

  pub fn testing() {
    let x = vec![[1, 23, 4]];
    // .to_owned();
  }
}

///
/// World is the master container for all worldly things.
/// Like the player, enemies, map data, bullets.
///
/// Neat.
///
pub struct World {
  pub player: Player,
  pub mobs: Vec<Mob>,
  pub bullet: HashMap<String, Bullet>,
  pub map: Map,
  pub plane: DVec2,
}

impl World {
  pub fn new() -> Self {
    World {
      player: Player::new(),
      mobs: vec![],
      bullet: HashMap::new(),
      map: Map::new(),
      plane: DVec2::new(0.0, 0.66),
    }
  }

  fn do_player_controls(&mut self, delta: f64, window: &WinHandler) {
    let move_speed = delta * 5.0;
    let mut moving = false;

    if window.forward_down {
      if self.map.data
        [(self.player.position.x + self.player.direction.x * move_speed).floor() as usize]
        [(self.player.position.y).floor() as usize]
        == 0
      {
        self.player.position.x += self.player.direction.x * move_speed;
      }
      if self.map.data[(self.player.position.x).floor() as usize]
        [(self.player.position.y + self.player.direction.y * move_speed).floor() as usize]
        == 0
      {
        self.player.position.y += self.player.direction.y * move_speed;
      }
      moving = true
    }

    if window.backward_down {
      if self.map.data
        [(self.player.position.x - self.player.direction.x * move_speed).floor() as usize]
        [(self.player.position.y).floor() as usize]
        == 0
      {
        self.player.position.x -= self.player.direction.x * move_speed;
      }
      if self.map.data[(self.player.position.x).floor() as usize]
        [(self.player.position.y - self.player.direction.y * move_speed).floor() as usize]
        == 0
      {
        self.player.position.y -= self.player.direction.y * move_speed;
      }
      moving = true;
    }

    if window.right_down {
      if self.map.data[(self.player.position.x + self.plane.x * move_speed).floor() as usize]
        [(self.player.position.y).floor() as usize]
        == 0
      {
        self.player.position.x += self.plane.x * move_speed;
      }
      if self.map.data[(self.player.position.x).floor() as usize]
        [(self.player.position.y + self.plane.y * move_speed).floor() as usize]
        == 0
      {
        self.player.position.y += self.plane.y * move_speed;
      }
      moving = true;
    }

    if window.left_down {
      if self.map.data[(self.player.position.x - self.plane.x * move_speed).round() as usize]
        [(self.player.position.y).floor() as usize]
        == 0
      {
        self.player.position.x -= self.plane.x * move_speed;
      }
      if self.map.data[(self.player.position.x).floor() as usize]
        [(self.player.position.y - self.plane.y * move_speed).floor() as usize]
        == 0
      {
        self.player.position.y -= self.plane.y * move_speed;
      }
      moving = true;
    }

    // println!("{}", window.mouse_delta);

    let mouse_delta = window.mouse_delta;
    let rot_speed = mouse_delta.x;
    let old_dir_x = self.player.direction.x;
    self.player.direction.x =
      self.player.direction.x * (-rot_speed).cos() - self.player.direction.y * (-rot_speed).sin();
    self.player.direction.y =
      old_dir_x * (-rot_speed).sin() + self.player.direction.y * (-rot_speed).cos();
    let old_plane_x = self.plane.x.clone();
    self.plane.x = self.plane.x * (-rot_speed).cos() - self.plane.y * (-rot_speed).sin();
    self.plane.y = old_plane_x * (-rot_speed).sin() + self.plane.y * (-rot_speed).cos();
  }

  ///
  /// What the world will do on each tick.
  ///
  pub fn on_tick(&mut self, delta: f64, window: &WinHandler) {
    // println!("tick tock {}", delta)

    self.do_player_controls(delta, window);
  }
}
