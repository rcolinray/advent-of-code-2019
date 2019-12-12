use std::fs::File;
use std::io::prelude::*;
use std::iter::repeat;

pub type Layer = Vec<u32>;

pub const BLACK: u32 = 0;
#[allow(dead_code)]
pub const WHITE: u32 = 1;
pub const CLEAR: u32 = 2;

pub struct Image {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
}

impl Image {
    pub fn from_file(filename: &str, width: usize, height: usize) -> Self {
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        Image::from_string(&contents, width, height)
    }

    pub fn from_string(string: &str, width: usize, height: usize) -> Self {
        let pixels_per_layer = width * height;
        let pixels = string
            .chars()
            .map(|c| {
                assert!(c.is_digit(10));
                c.to_digit(10).unwrap()
            })
            .collect::<Vec<_>>();
        assert!(pixels.len() % pixels_per_layer == 0);
        let layers = (0..pixels.len())
            .step_by(pixels_per_layer)
            .map(|index| Vec::from(&pixels[index..index + pixels_per_layer]))
            .collect::<Vec<_>>();
        Image {
            width,
            height,
            layers,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    #[allow(dead_code)]
    pub fn num_layers(&self) -> usize {
        self.layers.len()
    }

    pub fn layers_iter(&self) -> std::slice::Iter<Layer> {
        self.layers.iter()
    }

    pub fn pixels_per_layer(&self) -> usize {
        self.width * self.height
    }

    pub fn decode(&self) -> Layer {
        let mut output = repeat(CLEAR)
            .take(self.pixels_per_layer())
            .collect::<Vec<_>>();

        for pixel_idx in 0..output.len() {
            let pixel = self
                .layers
                .iter()
                .map(|layer| layer[pixel_idx])
                .find(|pixel| *pixel != CLEAR);

            match pixel {
                Some(color) => output[pixel_idx] = color,
                None => (),
            };
        }

        output
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_image_from_string() {
        let string = "123456789012";
        let image = Image::from_string(string, 3, 2);
        assert_eq!(image.num_layers(), 2);
    }

    #[test]
    fn test_decode() {
        let string = "0222112222120000";
        let image = Image::from_string(string, 2, 2);
        let layer = image.decode();
        assert_eq!(layer, vec![0, 1, 1, 0]);
    }
}
