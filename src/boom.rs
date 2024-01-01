use std::time::Duration;

use spin_sleep::LoopHelper;

use self::{renderer::Renderer, win_handler::WinHandler, world::World};

mod renderer;
mod win_handler;
mod world;

pub struct Boom {
  window: WinHandler,
  renderer: Renderer,
  world: World,
  should_close: bool,
  loop_helper: LoopHelper,
  delta: f64,
  current_fps: f64,
}

impl Boom {
  pub fn new() -> Self {
    return Boom {
      window: WinHandler::new(),
      renderer: Renderer::new(),
      world: World::new(),
      should_close: false,
      loop_helper: LoopHelper::builder()
        .report_interval(Duration::new(1, 0))
        .build_with_target_rate(60.0),
      delta: 0.0,
      current_fps: 0.0,
    };
  }

  ///
  /// Automatically poll the FPS counter from spin_sleep.
  ///
  fn update_fps(&mut self) {
    if let Some(fps) = self.loop_helper.report_rate() {
      self.current_fps = fps;
    }
  }

  ///
  /// Main logic of Boom.
  ///
  fn main(&mut self) {
    self.delta = self.loop_helper.loop_start_s();

    self.window.poll();

    if self.window.quit_received {
      self.should_close = true;
    }

    // self.world.on_tick(self.delta);

    self.update_fps();

    self
      .window
      .change_title(format!("Boom | delta: {} | FPS: {}", self.delta, self.current_fps).as_str());

    self.renderer.draw(&mut self.window, &mut self.world);

    self.loop_helper.loop_sleep();
  }

  ///
  /// Entry point to Boom.
  ///
  pub fn enter_main_loop(&mut self) {
    while !self.should_close {
      self.main();
    }
  }
}

impl Drop for Boom {
  fn drop(&mut self) {
    println!("Boom dropped!")
  }
}
