use std::borrow::Cow;
use serde::Deserialize;

use assets::AssetImage;

#[derive(AssetImage)]
#[folder = "images"]
struct Asset;

#[derive(Deserialize)]
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

    pub fn create_from_name(name: &str) -> Image {
        let image_struct_json: String = match Asset::get(name) {
            Some(cow) => match cow {
                Cow::Borrowed(json_str) => json_str.into(),
                Cow::Owned(json_str) => json_str.into(),
            },
            _ => panic!("Could not read image data."),
        };
        serde_json::from_str(&image_struct_json).unwrap()
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
