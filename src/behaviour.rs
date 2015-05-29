pub trait Behaviour : Send + Sync {
    fn update(&mut self);
}

