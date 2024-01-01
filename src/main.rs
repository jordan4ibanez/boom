use std::{cell::RefCell, rc::Rc};

use boom::Boom;

mod boom;

fn main() {
  // Move Boom into the heap. Then run it.
  Rc::new(RefCell::new(Boom::new()))
    .as_ref()
    .borrow_mut()
    .enter_main_loop();
}
