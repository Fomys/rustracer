pub trait Integrator {
    fn preprocess(&mut self);
    fn render(&mut self);
}
