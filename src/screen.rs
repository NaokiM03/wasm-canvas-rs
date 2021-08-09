use std::cmp;

use crate::image::Image;
use crate::color::Rgba;

pub struct Screen {
    bytes: Vec<u8>,
    width: usize,
    height: usize,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        Screen {
            bytes: vec![0; 4 * width * height],
            width: width,
            height: height,
        }
    }

    pub fn pointer(&self) -> *const u8 {
        self.bytes.as_ptr()
    }

    pub fn size(&self) -> usize {
        self.bytes.len()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn coordinates2index(&self, x: usize, y: usize) -> usize {
        x + &self.width * y
    }

    pub fn draw(&mut self, image: &Image, x: usize, y: usize) {
        let screen_index_init = self.coordinates2index(x, y);
        let image_paste_width = cmp::min(self.width - x, image.width);
        let image_paste_height = cmp::min(self.height - y, image.height);

        for h in 0..image_paste_height {
            for w in 0..image_paste_width {
                let screen_index = screen_index_init + self.coordinates2index(w, h);
                let image_index = image.coordinates2index(w, h);

                assert_eq!(image.bytes[4 * image_index + 3], Rgba::max_value());

                self.bytes[4 * screen_index + 0] = image.bytes[4 * image_index + 0];
                self.bytes[4 * screen_index + 1] = image.bytes[4 * image_index + 1];
                self.bytes[4 * screen_index + 2] = image.bytes[4 * image_index + 2];
                self.bytes[4 * screen_index + 3] = image.bytes[4 * image_index + 3];
            }
        }
    }
}
