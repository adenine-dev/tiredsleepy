struct Meta {
    data: u64,
}

impl Meta {
    pub fn idx(self) -> u16 {
        (self.data << 48) as u16
    }
}

struct Page<T> {
    data: [T; 4096],
    meta: [Meta; 4096],
}

pub struct Slotmap<T> {
    pages: std::vec::Vec<Page<T>>,
}

impl<T> Slotmap<T> {
    pub fn new() -> Self {
        Self { pages: vec![] }
    }

    pub fn insert<U: Into<T>>(&self, x: U) {}
}
