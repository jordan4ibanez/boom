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
