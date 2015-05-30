use std::sync::{Arc, Mutex};

use glium::vertex::VertexBufferAny;
use glium::index::IndexBuffer;
use glium::Program;

use transform::Transform;

pub struct RenderObject {
    pub transforms: Vec<Arc<Mutex<Transform>>>,
    pub vertices: VertexBufferAny,
    pub indices: IndexBuffer<u16>,
    pub program: Program,
}
