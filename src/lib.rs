#[macro_use]
extern crate glium;

extern crate time;

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::collections::HashMap;

use glium::Display;
use glium::DisplayBuild;
use glium::Surface;
use glium::vertex::VertexBufferAny;
use glium::vertex::VertexBuffer;
use glium::glutin;

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
    behaviour_sender: Option<Sender<Box<Behaviour>>>,
    render_objects: HashMap<&'static str, RenderObject>,
}

impl Oxen {
    pub fn new(width: f32, height: f32) -> Oxen {
        let mut oxen = Oxen {
            display: glutin::WindowBuilder::new()
                .with_dimensions(width as u32, height as u32)
                .with_title(format!("Oxen game engine"))
                .with_vsync()
                .build_glium()
                .unwrap(),
            width: width,
            height: height,
            camera: (0., 0.),
            behaviour_sender: None,
            render_objects: HashMap::new(),
        };
        oxen.load_models();
        oxen.game_loop();
        oxen
    }

    pub fn set_camera(&mut self, coords: (f32, f32)) {
        self.camera = coords;
    }

    pub fn add_behaviour(&mut self, behaviour: Box<Behaviour>) {
        match self.behaviour_sender {
            Some(ref sender) => sender.send(behaviour).unwrap(),
            None => panic!("Oxen's behaviour sender has not been initialised! This is a bug in oxen :(")
        }
    }

    pub fn attach_render_object(&mut self, transform: Arc<Mutex<Transform>>, render_object_name: &str) -> Result<(), &'static str> {
        match self.render_objects.get_mut(render_object_name) {
            Some(render_object) => {
                render_object.transforms.push(transform);
                Ok(())
            },
            None => {
                Err("No such render object: {}") //TODO
            }
        }
    }

    pub fn render_loop(&self) {
        let mut frame_count = 0u16;
        let mut previous_second = 0u64;
        loop {
            if time::precise_time_ns() - previous_second > 1000000000 {
                println!("Render fps: {}", frame_count);
                frame_count = 0;
                previous_second = time::precise_time_ns();
            }
            frame_count += 1;

            let uniforms = uniform!{ view_transform: self.view_transform() };

            let mut frame = self.display.draw();
            frame.clear_color(1.0, 1.0, 1.0, 1.0);

            for render_object in self.render_objects.values() {
                let instances = self.instances(&render_object.transforms);

                frame.draw(
                    (&render_object.vertices, instances.per_instance_if_supported().unwrap()),
                    &render_object.indices,
                    &render_object.program,
                    &uniforms,
                    &std::default::Default::default()
                ).unwrap();
            }

            frame.finish();

            for event in self.display.poll_events() {
                match event {
                    glutin::Event::Closed => return,
                    _ => ()
                }
            }
        }
    }

    fn load_models(&mut self) {
        let square = self.square();
        self.render_objects.insert("square", square);
    }

    fn game_loop(&mut self) {
        let (sender, receiver) = channel();
        self.behaviour_sender = Some(sender);
        let mut behaviours = Vec::<Box<Behaviour>>::new();

        thread::spawn(move || {
            let mut frame_count = 0u32;
            let mut previous_second = 0u64;
            loop {
                if time::precise_time_ns() - previous_second > 1000000000 {
                    println!("Update fps: {}", frame_count);
                    frame_count = 0;
                    previous_second = time::precise_time_ns();
                }
                frame_count += 1;

                loop {
                    match receiver.try_recv() {
                        Ok(behaviour) => behaviours.push(behaviour),
                        Err(_) => break
                    };
                }

                for behaviour in behaviours.iter_mut() {
                    behaviour.update();
                }
            }
        });
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

