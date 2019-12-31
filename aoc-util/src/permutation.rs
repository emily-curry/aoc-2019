pub struct Permutation<T>
where
  T: Copy,
{
  arr: Vec<T>,
  out: Vec<Vec<T>>,
}

impl<T> Permutation<T>
where
  T: Copy,
{
  pub fn calculate(input: &Vec<T>) -> Vec<Vec<T>> {
    let mut p = Permutation {
      arr: input.clone(),
      out: vec![],
    };
    p.permute(input.len());
    p.out
  }

  fn permute(&mut self, el: usize) {
    if el == 1 {
      self.out.push(self.arr.clone());
      return;
    }

    let mut i = 0;
    while i < el {
      self.permute(el - 1);

      if el % 2 == 0 {
        self.swap(i, el - 1);
      } else {
        self.swap(0, el - 1);
      }

      i += 1;
    }
  }

  fn swap(&mut self, a: usize, b: usize) {
    let x = self.arr[a];
    self.arr[a] = self.arr[b];
    self.arr[b] = x;
  }
}
