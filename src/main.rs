mod constvals;
mod model;
mod view;

fn main() {
    let model = model::Model::new();
    model.debug();
    let mut view = view::View::new();
    view.update(model);
    view.draw();
}
