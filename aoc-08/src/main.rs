use std::fs;

const WIDTH: u32 = 25;
const HEIGHT: u32 = 6;

fn main() {
    let input = fs::read_to_string("aoc-08/input.txt").unwrap();
    let image = Image::from_string(WIDTH, HEIGHT, &input);
    validation(&image);
}

fn validation(image: &Image) {
    let zero_digit_counts = image.count_digits(0);
    let (min_idx, _min_val) =
        zero_digit_counts
            .iter()
            .enumerate()
            .fold((0, zero_digit_counts[0]), |acc, x| {
                if *x.1 < acc.1 {
                    (x.0, *x.1)
                } else {
                    acc
                }
            });
    let one_digit_count = image.layers[min_idx].count_digit(1);
    let two_digit_count = image.layers[min_idx].count_digit(2);
    let result = one_digit_count * two_digit_count;
    println!("Validation: {:?}", result);
}

struct ImageLayer {
    data: Vec<Vec<u32>>,
}

impl ImageLayer {
    pub fn new() -> Self {
        ImageLayer { data: Vec::new() }
    }

    pub fn count_digit(&self, digit: u32) -> u32 {
        self.data.iter().fold(0, |acc, x| {
            acc + x
                .iter()
                .fold(0, |iacc, i| if *i == digit { iacc + 1 } else { iacc })
        })
    }
}

struct Image {
    width: u32,
    height: u32,
    layers: Vec<ImageLayer>,
}

impl Image {
    pub fn from_string(width: u32, height: u32, input: &str) -> Self {
        let mut layers: Vec<ImageLayer> = Vec::new();
        let mut numbers = input.chars().map(|x| x.to_digit(10).unwrap()).peekable();
        loop {
            let mut layer = ImageLayer::new();
            for h in 0..height {
                layer.data.push(Vec::new());
                for _w in 0..width {
                    if let Some(t) = numbers.next() {
                        layer.data[h as usize].push(t);
                    }
                }
            }
            layers.push(layer);
            if numbers.peek().is_none() {
                break;
            }
        }
        Image {
            width,
            height,
            layers,
        }
    }

    pub fn count_digits(&self, digit: u32) -> Vec<u32> {
        self.layers.iter().map(|x| x.count_digit(digit)).collect()
    }
}
