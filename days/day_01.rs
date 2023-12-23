use std::fs;

fn main() {
    let file = String::from("./resources/input_01.txt");
    let input = fs::read_to_string(&file).expect(&format!("Unable to open file {}", file.as_str()));

    let mut total = 0;
    for line in input.lines() {
        let mut digit_vec = Vec::new();
        for ch in line.chars() {
            if ch.is_ascii_digit() {
                digit_vec.push(ch.to_digit(10).unwrap());
            }
        }

        let val = digit_vec.first().unwrap() * 10 + digit_vec.last().unwrap();
        total = total + val;
    }

    println!("Solution 1: {}", total);
}
