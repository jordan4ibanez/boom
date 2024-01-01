use std::collections::{hash_map, HashMap};

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

struct Bullet {}

impl Bullet {
  pub fn new() -> Self {
    Bullet {}
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
