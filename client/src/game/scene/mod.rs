pub mod loading_scene;

pub trait Scene {
    fn render(&self);
}
