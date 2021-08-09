pub struct Image {
    pub bytes: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            bytes: vec![0; 4 * width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn coordinates2index(&self, x: usize, y: usize) -> usize {
        x + &self.width * y
    }
}
