use glium::Display;
use glium::vertex::VertexBufferAny;
use glium::vertex::VertexBuffer;
use glium::index::IndexBuffer;
use glium::index::PrimitiveType::TrianglesList;
use glium::Program;

use shaders;
use render_object::RenderObject;

pub fn cube(display: &Display) -> RenderObject {
    RenderObject {
        transforms: Vec::new(),
        vertices: vertices(display),
        indices: indices(display),
        program: Program::from_source(display, shaders::VERTEX, shaders::FRAGMENT, None).unwrap(),
    }
}

fn vertices(display: &Display) -> VertexBufferAny {
    #[derive(Copy, Clone)]
    struct Vertex {
        vertex_position: [f32; 3],
        vertex_color: [f32; 3],
    }

    implement_vertex!(Vertex, vertex_position, vertex_color);

    let colour = [0.2, 0.2, 0.2];

    VertexBuffer::new(display,
        vec![
            //Near plane
            Vertex { vertex_position: [ -0.5,  0.5, -0.5], vertex_color: colour }, //0: Top left
            Vertex { vertex_position: [  0.5,  0.5, -0.5], vertex_color: colour }, //1: Top right
            Vertex { vertex_position: [ -0.5, -0.5, -0.5], vertex_color: colour }, //2: Bottom left
            Vertex { vertex_position: [  0.5, -0.5, -0.5], vertex_color: colour }, //3: Bottom right
            //Far plane
            Vertex { vertex_position: [ -0.5,  0.5, 0.5], vertex_color: colour },  //4: Top left
            Vertex { vertex_position: [  0.5,  0.5, 0.5], vertex_color: colour },  //5: Top right
            Vertex { vertex_position: [ -0.5, -0.5, 0.5], vertex_color: colour },  //6: Bottom left
            Vertex { vertex_position: [  0.5, -0.5, 0.5], vertex_color: colour },  //7: Bottom right
        ]
    ).into_vertex_buffer_any()
}

fn indices(display: &Display) -> IndexBuffer<u16> {
    IndexBuffer::new(display, TrianglesList, vec![
        0, 1, 2, 1, 3, 2, //front face
        1, 5, 3, 5, 7, 3, //right face
        5, 4, 7, 4, 6, 7, //rear face
        4, 0, 6, 0, 2, 6, //left face
        4, 5, 0, 5, 1, 0, //top face
        2, 3, 6, 3, 7, 6, //bot face
    ])
}

