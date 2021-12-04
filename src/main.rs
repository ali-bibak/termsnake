mod constvals;
mod model;
mod view;
mod controller;

use controller::Controller;

fn main() {
    let mut controller = Controller::new();
    controller.debug();
    for _ in 0..20 {
        controller.draw();
    }
}
