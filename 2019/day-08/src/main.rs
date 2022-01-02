use common::{default_puzzle, Puzzle};

struct Layer {
    pixels: Vec<Vec<u8>>,
}

impl Layer {
    fn new(width: usize, height: usize, nums: &[u8]) -> Layer {
        let mut pixels = vec![vec![0; width]; height];
        let mut i = 0;
        #[allow(clippy::needless_range_loop)]
        for y in 0..height {
            for x in 0..width {
                pixels[y][x] = nums[i];
                i += 1;
            }
        }
        Layer { pixels }
    }

    fn count_pixels(&self, value: u8) -> u64 {
        self.pixels
            .iter()
            .flatten()
            .filter(|p| **p == value)
            .count() as u64
    }
}

struct Image {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
}

impl Image {
    fn new(width: usize, height: usize, nums: &[u8]) -> Image {
        let num_pixels = width * height;
        let (mut start, mut end) = (0, num_pixels);
        let mut layers = vec![];
        while end < nums.len() {
            layers.push(Layer::new(width, height, &nums[start..end].to_vec()));
            start += num_pixels;
            end += num_pixels;
        }
        Image {
            width,
            height,
            layers,
        }
    }

    fn get_full_image(&self) -> Layer {
        let twos = vec![2; self.width * self.height];
        let mut image = Layer::new(self.width, self.height, &twos);
        for y in 0..self.height {
            for x in 0..self.width {
                for layer in self.layers.iter() {
                    if image.pixels[y][x] == 2 {
                        image.pixels[y][x] = layer.pixels[y][x];
                    }
                }
            }
        }
        image
    }

    fn get_layer_with_fewest(&self, value: u8) -> &Layer {
        let mut min_layer = 0;
        let mut min_count = self.width * self.height;
        for (i, layer) in self.layers.iter().enumerate() {
            let count = layer.count_pixels(value) as usize;
            if count < min_count {
                min_count = count;
                min_layer = i;
            }
        }
        &self.layers[min_layer]
    }
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Space Image Format");
    puzzle.set_part1(
        "product of ones and twos in layer with fewest zeroes",
        |reader| {
            let pixels = reader.digit_line(10);
            let image = Image::new(25, 6, &pixels);
            let layer = image.get_layer_with_fewest(0);
            layer.count_pixels(1) * layer.count_pixels(2)
        },
    );
    puzzle.set_part2(
        "the message is printed above; the following is not the answer",
        |reader| {
            let pixels = reader.digit_line(10);
            let image = Image::new(25, 6, &pixels);
            let full_image = image.get_full_image();
            for y in 0..6 {
                for x in 0..25 {
                    let pixel = match full_image.pixels[y][x] {
                        0 => "⚫",
                        1 => "⚪",
                        2 => "⚫",
                        _ => " ",
                    };
                    print!("{}", pixel);
                }
                println!();
            }
            println!("--> CJZHR");
            0
        },
    );
    puzzle
}

fn main() {
    get_puzzle().run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let image = Image::new(3, 2, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
        let layer = image.get_layer_with_fewest(0); // first layer
        assert_eq!(layer.pixels, vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(layer.count_pixels(0), 0);
        assert_eq!(layer.count_pixels(1), 1);
        assert_eq!(layer.count_pixels(2), 1);
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(2159);
    }

    #[test]
    fn test_part2() {
        // The answer is visual.
        get_puzzle().test_part2(0);
    }
}
