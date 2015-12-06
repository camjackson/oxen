use glium::Display;
use render_object::RenderObject;

mod square;
mod cube;

pub fn square(display: &Display) -> RenderObject {
    square::square(display)
}

pub fn cube(display: &Display) -> RenderObject {
    cube::cube(display)
}

