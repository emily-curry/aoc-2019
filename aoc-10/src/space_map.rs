use super::char_enum::CharEnum;

pub struct SpaceMap<T>
where
  T: CharEnum,
{
  map_data: Vec<Vec<T>>,
}

impl<T> SpaceMap<T>
where
  T: CharEnum + PartialEq,
{
  pub fn from_string(input: &str) -> Self {
    let map_data = input
      .lines()
      .map(|line| line.chars().map(|c| T::from_char(&c).unwrap()).collect())
      .collect();
    SpaceMap { map_data }
  }

  pub fn print(&self) {
    let map_str = self.map_data.iter().fold(String::from("\n"), |mut acc, y| {
      let line = y.iter().fold(String::from(" "), |mut iacc, x| {
        iacc.push(x.to_char());
        iacc
      });
      acc.push_str(&line);
      acc.push('\n');
      acc
    });
    println!("{}", map_str);
  }

  pub fn set_point(&mut self, point: &(usize, usize), value: T) {
    self.map_data[point.1][point.0] = value;
  }

  pub fn at_point(&self, point: &(usize, usize)) -> &T {
    &self.map_data[point.1][point.0]
  }

  pub fn points_one_of(&self, types: Vec<T>) -> Vec<(usize, usize)> {
    let mut result = Vec::<(usize, usize)>::new();
    for y in 0..self.map_data.len() {
      for x in 0..self.map_data[y].len() {
        let p = (x, y);
        for t in &types {
          if self.at_point(&p) == t {
            result.push(p);
            break;
          }
        }
      }
    }
    result
  }
}
