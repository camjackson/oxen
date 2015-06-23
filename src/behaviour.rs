use glium::glutin::VirtualKeyCode;

pub trait Behaviour : Send + Sync {
    fn update(&mut self, &Fn(VirtualKeyCode) -> bool);
}

