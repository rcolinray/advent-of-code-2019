mod image;

use image::{Image, Layer, BLACK};

fn main() {
    let image = Image::from_file("./password.sif", 25, 6);
    let stats = image
        .layers_iter()
        .map(|layer| LayerStats::from_layer(layer))
        .min_by(|stats1, stats2| stats1.num_zeros.cmp(&stats2.num_zeros))
        .unwrap();
    let ones_by_twos = stats.num_ones * stats.num_twos;
    println!("part 1: {:?}", ones_by_twos);

    let output = render(&image);
    println!("part 2:\n{}", output);
}

#[derive(Default, Debug)]
struct LayerStats {
    num_zeros: usize,
    num_ones: usize,
    num_twos: usize,
}

impl LayerStats {
    fn from_layer(layer: &Layer) -> Self {
        let mut stats: LayerStats = Default::default();
        for byte in layer {
            match &byte {
                0 => stats.num_zeros += 1,
                1 => stats.num_ones += 1,
                2 => stats.num_twos += 1,
                _ => (),
            }
        }
        stats
    }
}

const PADDING: usize = 1;

fn render(image: &Image) -> String {
    let layer = image.decode();

    let output_width = image.get_width() + (2 * PADDING);
    let output_height = image.get_height() + (2 * PADDING);
    let output_size = output_width * output_height;

    let mut output = String::with_capacity(output_size);

    for _ in 0..output_width {
        output.push('█');
    }
    output.push('\n');

    for y in 0..image.get_height() {
        output.push('█');
        for x in 0..image.get_width() {
            let layer_idx = y * image.get_width() + x;
            let color = layer[layer_idx];
            let pixel = match color {
                BLACK => '█',
                _ => ' ',
            };
            output.push(pixel);
        }
        output.push('█');
        output.push('\n');
    }

    for _ in 0..output_width {
        output.push('█');
    }
    output.push('\n');

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_layer_stats() {
        let layer = vec![0, 1, 1, 2, 2, 2];
        let stats = LayerStats::from_layer(&layer);
        assert_eq!(stats.num_zeros, 1);
        assert_eq!(stats.num_ones, 2);
        assert_eq!(stats.num_twos, 3);
    }

    #[test]
    fn test_render() {
        let image = Image::from_string("0222112222120000", 2, 2);
        let output = render(&image);
        assert_eq!(output, "████\n██ █\n█ ██\n████\n");
    }
}
