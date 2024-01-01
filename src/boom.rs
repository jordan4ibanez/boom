use self::win::Win;

mod win;

pub struct Boom {
  window: Win,
}

impl Boom {
  pub fn new() -> Self {
    return Boom { window: Win::new() };
  }

  pub fn main(&mut self) {}
}

impl Drop for Boom {
  fn drop(&mut self) {
    println!("Boom dropped!")
  }
}
