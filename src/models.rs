use glium::Display;
use glium::vertex::VertexBufferAny;
use glium::vertex::VertexBuffer;
use glium::index::IndexBuffer;
use glium::index::PrimitiveType::TrianglesList;
use glium::Program;

use shaders;
use render_object::RenderObject;

pub fn square(display: &Display) -> RenderObject {
    RenderObject {
        transforms: Vec::new(),
        vertices: square_vertices(display),
        indices: square_indices(display),
        program: Program::from_source(display, shaders::VERTEX, shaders::FRAGMENT, None).unwrap(),
    }
}

fn square_vertices(display: &Display) -> VertexBufferAny {
    #[derive(Copy, Clone)]
    struct Vertex {
        vertex_position: [f32; 2],
        vertex_color: [f32; 3],
    }

    implement_vertex!(Vertex, vertex_position, vertex_color);

    let colour = [0.2, 0.2, 0.2];

    VertexBuffer::new(display,
        vec![
            Vertex { vertex_position: [ -0.5,  0.5], vertex_color: colour },
            Vertex { vertex_position: [  0.5,  0.5], vertex_color: colour },
            Vertex { vertex_position: [  0.5, -0.5], vertex_color: colour },
            Vertex { vertex_position: [ -0.5, -0.5], vertex_color: colour },
        ]
    ).into_vertex_buffer_any()
}

fn square_indices(display: &Display) -> IndexBuffer<u16> {
    IndexBuffer::new(display, TrianglesList, vec![0, 1, 2, 0, 2, 3])
}

