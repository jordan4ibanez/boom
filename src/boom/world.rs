use std::collections::HashMap;

use glam::Vec2;

struct Mob {
  alive: bool,
  position: Vec2,
  yaw: f64,
  sprite: usize,
}
impl Mob {
  pub fn new(position: Vec2) -> Self {
    Mob {
      alive: true,
      position,
      yaw: 0.0,
      sprite: 0,
    }
  }
}

struct Bullet {
  position: Vec2,
  direction: Vec2,
}

impl Bullet {
  pub fn new(position: Vec2, direction: Vec2) -> Self {
    Bullet {
      position,
      direction,
    }
  }
}

pub struct World {
  mobs: Vec<Mob>,
  bullet: HashMap<String, Bullet>,
}

impl World {
  pub fn new() -> Self {
    World {
      mobs: vec![],
      bullet: HashMap::new(),
    }
  }

  pub fn on_tick(&mut self, delta: f64) {
    println!("tick tock {}", delta)
  }
}
