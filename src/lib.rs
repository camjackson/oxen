#[macro_use]
extern crate glium;
extern crate glutin;
extern crate time;

use std::thread;
use std::sync::{Arc, Mutex};

use glium::Display;
use glium::DisplayBuild;
use glium::Surface;
use glium::vertex::VertexBufferAny;
use glium::vertex::VertexBuffer;

mod models;
mod shaders;
mod render_object;
mod transform;
mod behaviour;

pub use self::render_object::RenderObject;
pub use self::transform::Transform;
pub use self::behaviour::Behaviour;

pub struct Oxen {
    display: glium::Display,
    width: f32,
    height: f32,
    camera: (f32, f32),
    behaviour: Option<Box<Behaviour>>,
    render_object: Option<RenderObject>,
}

impl Oxen {
    pub fn new(width: f32, height: f32) -> Oxen {
        Oxen {
            display: glutin::WindowBuilder::new()
                .with_dimensions(width as u32, height as u32)
                .with_title(format!("Oxen game engine"))
                .with_vsync()
                .build_glium()
                .unwrap(),
            width: width,
            height: height,
            camera: (0., 0.),
            behaviour: None,
            render_object: None,
        }
    }

    pub fn set_camera(&mut self, coords: (f32, f32)) {
        self.camera = coords;
    }

    pub fn set_behaviour(&mut self, behaviour: Box<Behaviour>) {
        self.behaviour = Some(behaviour);
    }

    pub fn set_render_object(&mut self, render_object: RenderObject) {
        self.render_object = Some(render_object);
    }

    pub fn game_loop(&mut self, updates_per_second: u32) {
        let mut behaviour = match self.behaviour.take(){
            Some(b) => b,
            None => return
        };
        thread::spawn(move || {
            let mut frame_count = 0u16;
            let mut previous_second = 0u64;
            loop {
                if time::precise_time_ns() - previous_second > 1000000000 {
                    println!("Update fps: {}", frame_count);
                    frame_count = 0;
                    previous_second = time::precise_time_ns();
                }
                frame_count += 1;
                thread::sleep_ms(1000 / updates_per_second);
                behaviour.update();
            }
        });
    }

    pub fn render_loop(&mut self) {
        let render_object = match self.render_object.take() {
            Some(r) => r,
            None => return
        };
        let mut frame_count = 0u16;
        let mut previous_second = 0u64;
        loop {
            if time::precise_time_ns() - previous_second > 1000000000 {
                println!("Render fps: {}", frame_count);
                frame_count = 0;
                previous_second = time::precise_time_ns();
            }
            frame_count += 1;

            let instances = self.instances(&render_object.transforms);
            let uniforms = uniform! { view_transform: self.view_transform() };

            let mut frame = self.display.draw();
            frame.clear_color(1.0, 1.0, 1.0, 1.0);
            frame.draw(
                (&render_object.vertices, instances.per_instance_if_supported().unwrap()),
                &render_object.indices,
                &render_object.program,
                &uniforms,
                &std::default::Default::default()
            ).unwrap();
            frame.finish();

            for event in self.display.poll_events() {
                match event {
                    glutin::Event::Closed => return,
                    _ => ()
                }
            }
        }
    }

    fn instances(&self, transforms: &Vec<Arc<Mutex<Transform>>>) -> VertexBufferAny {
        #[derive(Copy, Clone)]
        struct ModelTransform {
            model_position: [f32; 2],
            model_scale: f32,
        }

        implement_vertex!(ModelTransform, model_position, model_scale);

        let mut data = Vec::new();
        for transform in transforms.iter() {
            let mutex = transform.clone();
            let t = mutex.lock().unwrap();
            if t.visible {
                data.push(ModelTransform {
                    model_position: [t.x, t.y],
                    model_scale: t.scale
                })
            }
        }
        VertexBuffer::new(&self.display, data).into_vertex_buffer_any()
    }

    fn view_transform(&self) -> [[f32; 4]; 4] {
        let (x, y) = self.camera;
        [
            [ 1.0 / self.width, 0.0              , 0.0, 0.0],
            [ 0.0             , 1.0 / self.height, 0.0, 0.0],
            [ 0.0             , 0.0              , 1.0, 0.0],
            [-x               , -y               , 0.0, 1.0f32]
        ]
    }

    pub fn square(&self) -> RenderObject {
        models::square(&self.display)
    }
}

