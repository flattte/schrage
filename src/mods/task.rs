use std::fmt::Display;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Task {
    pub r: u32, // ready time
    pub p: u32, // working time
    pub q: u32, // cooldown time
}

impl Task {
    #[allow(unused)]
    pub fn new(r: u32, p: u32, q: u32) -> Task {
        Task { r, p, q }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.p, self.q)
    }
}