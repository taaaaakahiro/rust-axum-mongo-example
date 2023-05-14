pub mod user;

#[derive(Debug)]
pub struct Id {
    pub value: i32,
}

impl Id {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}
