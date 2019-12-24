pub struct PasswordChecker {
  password: i32,
  end: i32,
  found: Vec<i32>,
}

impl PasswordChecker {
  pub fn new(start: i32, end: i32) -> PasswordChecker {
    PasswordChecker {
      password: start,
      end,
      found: Vec::new(),
    }
  }

  pub fn find(&mut self) -> &Vec<i32> {
    while self.password <= self.end {
      if self.validate() {
        self.found.push(self.password);
      }
      self.password += 1;
    }
    &self.found
  }

  fn validate(&self) -> bool {
    self.validate_adjacent_duplicate() && self.validate_ascending()
  }

  fn validate_adjacent_duplicate(&self) -> bool {
    let mut prev: Option<i32> = None;
    let mut count = vec![1];
    for digit in self.digits() {
      if let Some(i) = prev {
        if i == digit {
          let len = count.len();
          count[len - 1] += 1;
        } else {
          count.push(1);
        }
      }
      prev = Some(digit);
    }
    for c in count {
      if c == 2 {
        return true;
      }
    }
    false
  }

  fn validate_ascending(&self) -> bool {
    let mut prev: Option<i32> = None;
    for digit in self.digits() {
      if let Some(i) = prev {
        if i > digit {
          return false;
        }
      }
      prev = Some(digit);
    }
    true
  }

  fn digits(&self) -> Vec<i32> {
    let mut i = self.password;
    let mut d = Vec::new();
    while i >= 1 {
      d.push(i % 10);
      i /= 10;
    }
    d.reverse();
    d
  }
}
