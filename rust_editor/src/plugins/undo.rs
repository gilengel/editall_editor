use rust_internal::plugin::Plugin;

use crate::actions::Action;

pub struct Undo<T> {
    pub stack: Vec<Box<dyn Action<T>>>
}

impl<T> Undo<T> {
    pub fn push(&mut self, action: Box<dyn Action<T>>) {
        self.stack.push(action);
    }
}


impl<T> Plugin<T> for Undo<T> where T: 'static {

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn mouse_down(&mut self, _mouse_pos: geo::Coordinate<f64>, _button: u32, _data: &mut T) {}

    fn mouse_move(
        &mut self,
        _mouse_pos: geo::Coordinate<f64>,
        _mouse_movement: geo::Coordinate<f64>,
        _data: &mut T,
    ) {
    }

    fn mouse_up(&mut self, _mouse_pos: geo::Coordinate<f64>, _button: u32, _data: &mut T) {}

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}