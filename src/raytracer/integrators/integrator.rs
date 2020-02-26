pub trait Integrator {
    fn preprocess(&mut self);
    fn render(&mut self, max_iteration: usize);
    fn next_frame(&mut self) {}
}
