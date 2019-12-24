mod password_checker;

use crate::password_checker::PasswordChecker;

const START: i32 = 172851;
const END: i32 = 675869;

fn main() {
    let mut pw = PasswordChecker::new(START, END);
    let results = pw.find();
    println!("{}", results.len());
}
