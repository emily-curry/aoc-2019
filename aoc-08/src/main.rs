use std::fs;

const WIDTH: u32 = 25;
const HEIGHT: u32 = 6;

fn main() {
    let input = fs::read_to_string("aoc-08/input.txt").unwrap();
    let image = Image::from_string(WIDTH, HEIGHT, &input);
    validation(&image);
    decode(&image);
}

fn decode(image: &Image) {
    let output = image.to_string();
    println!("{}", output);
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

    pub fn to_string(&self) -> String {
        let decoded = self.decode();
        let mut s = String::new();
        for y in &decoded.data {
            for x in y {
                match x {
                    0 => s.push(' '),
                    1 => s.push('█'),
                    _ => panic!(),
                }
            }
            if decoded.data.last().unwrap() != y {
                s.push('\n');
            }
        }
        s
    }

    fn decode(&self) -> ImageLayer {
        let mut layer = ImageLayer::new();
        for y in 0..self.height {
            layer.data.push(Vec::new());
            for x in 0..self.width {
                layer.data[y as usize].push(self.px_at(x as usize, y as usize))
            }
        }
        layer
    }

    fn px_at(&self, x: usize, y: usize) -> u32 {
        for layer in &self.layers {
            match layer.data[y][x] {
                0 => return 0,
                1 => return 1,
                2 => continue,
                _ => panic!("Unknown pixel color"),
            }
        }
        panic!("All layers are transparent")
    }
}
