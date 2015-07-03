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
use glium::glutin::VirtualKeyCode;

mod models;
mod shaders;
mod render_object;
mod transform;
mod behaviour;
mod camera;

pub use self::render_object::RenderObject;
pub use self::transform::Transform;
pub use self::behaviour::Behaviour;
pub use self::camera::Camera;

/// The main Oxen game engine object. This is responsible for creating the display,
/// and managing game and render objects. You'll use this for most (all?) of your
/// interactions with the engine.
pub struct Oxen {
    display: glium::Display,
    width: f32,
    height: f32,
    camera: Option<Arc<Mutex<Camera>>>,
    behaviour_sender: Option<Sender<Box<Behaviour>>>,
    render_objects: HashMap<&'static str, RenderObject>,
    keyboard_state: Arc<Mutex<HashMap<VirtualKeyCode, bool>>>,
}

impl Oxen {
    /// Game engine constructor
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
            camera: None,
            behaviour_sender: None,
            render_objects: HashMap::new(),
            keyboard_state: Arc::new(Mutex::new(HashMap::new())),
        };
        oxen.load_models();
        oxen.game_loop();
        oxen
    }

    pub fn set_camera(&mut self, camera: Arc<Mutex<Camera>>) {
        self.camera = Some(camera);
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

    pub fn render_loop(&mut self) {
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

            frame.finish().unwrap();

            for event in self.display.poll_events() {
                match event {
                    glutin::Event::Closed => return,
                    glutin::Event::KeyboardInput(state, _, some_keycode) => {
                        match (state, some_keycode) {
                            (_, None) => (),
                            (glutin::ElementState::Pressed, Some(keycode)) => {self.keyboard_state.lock().unwrap().insert(keycode, true);},
                            (glutin::ElementState::Released, Some(keycode)) => {self.keyboard_state.lock().unwrap().insert(keycode, false);},
                        }
                    },
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
        let (behaviour_sender, behaviour_receiver) = channel();
        self.behaviour_sender = Some(behaviour_sender);

        let mut behaviours = Vec::<Box<Behaviour>>::new();
        let keyboard_state = self.keyboard_state.clone();

        thread::spawn(move || {
            let mut frame_count = 0u32;
            let mut previous_second = 0u64;
            loop {
                if time::precise_time_ns() - previous_second > 1_000_000_000 {
                    println!("Update fps: {}", frame_count);
                    frame_count = 0;
                    previous_second = time::precise_time_ns();
                }
                frame_count += 1;

                loop {
                    match behaviour_receiver.try_recv() {
                        Ok(behaviour) => behaviours.push(behaviour),
                        Err(_) => break
                    };
                }

                let key_pressed = &|key: VirtualKeyCode| {
                    match keyboard_state.lock().unwrap().get(&key) {
                        Some(pressed) => *pressed,
                        None => false
                    }
                };
                for behaviour in behaviours.iter_mut() {
                    behaviour.update(key_pressed);
                }
            }
        });
    }

    fn instances(&self, transforms: &Vec<Arc<Mutex<Transform>>>) -> VertexBufferAny {
        #[derive(Copy, Clone)]
        struct ModelTransform {
            model_transform: [[f32; 4]; 4],
        }

        implement_vertex!(ModelTransform, model_transform);

        let mut data = Vec::new();
        for transform in transforms.iter() {
            let mutex = transform.clone();
            let t = mutex.lock().unwrap();
            if t.visible {
                data.push(ModelTransform {
                    model_transform: [
                        [t.scale_x, 0., 0., 0.],
                        [0., t.scale_y, 0., 0.],
                        [0., 0., t.scale_z, 0.],
                        [t.x, t.y, t.z, 1.],
                    ],
                })
            }
        }
        VertexBuffer::new(&self.display, data).into_vertex_buffer_any()
    }

    fn view_transform(&self) -> [[f32; 4]; 4] {
        let (x, y) = match self.camera {
            Some(ref c) => {
                let mutex = c.clone();
                let ref camera = mutex.lock().unwrap().transform;
                (camera.x, camera.y)
            },
            None => panic!("You must assign Oxen a Camera before starting the render loop")
        };
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

