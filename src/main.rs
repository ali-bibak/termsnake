mod constvals;
mod model;
mod view;
mod controller;

use controller::Controller;

fn main() {
    let mut controller = Controller::new();
    controller.debug();
    controller.draw();
}
