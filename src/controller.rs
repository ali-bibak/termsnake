use crate::model::Model;
use crate::view::View;

pub struct Controller {
    model: Model,
    view: View,
}

impl Controller {
    pub fn new() -> Self {
        let model = Model::new();
        let view = View::new();
        let controller = Controller {
            model,
            view,
        };
        return controller;
    }

    pub fn debug(&self) {
        self.model.debug();
    }

    pub fn draw(&mut self) {
        self.view.update(&self.model);
        self.view.clean();
        self.view.draw();
    }
}