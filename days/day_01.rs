use std::fs;

fn main() {
    let file = String::from("./resources/input_01.txt");
    let input = fs::read_to_string(&file).expect(&format!("Unable to open file {}", file.as_str()));

    /* Part 1 */
    let mut total = 0;
    for line in input.lines() {
        let mut digit_vec = Vec::new();
        for ch in line.chars() {
            if ch.is_digit(10) {
                digit_vec.push(ch.to_digit(10).unwrap());
            }
        }

        let val = digit_vec.first().unwrap() * 10 + digit_vec.last().unwrap();
        total = total + val;
    }

    println!("Solution 1: {}", total);

    /* Part 2 */
    total = 0;
    for line in input.lines() {
        let mut digit_vec = Vec::new();
        let mut num_str = String::new();
        for ch in line.chars() {
            if ch.is_digit(10) {
                digit_vec.push(ch.to_digit(10).unwrap());
                num_str.clear();
            } else {
                num_str.push(ch);
                if num_str.contains("one") {
                    digit_vec.push(1);
                    num_str.clear();
                } else if num_str.contains("two") {
                    digit_vec.push(2);
                    num_str.clear();
                } else if num_str.contains("three") {
                    digit_vec.push(3);
                    num_str.clear();
                } else if num_str.contains("four") {
                    digit_vec.push(4);
                    num_str.clear();
                } else if num_str.contains("five") {
                    digit_vec.push(5);
                    num_str.clear();
                } else if num_str.contains("six") {
                    digit_vec.push(6);
                    num_str.clear();
                } else if num_str.contains("seven") {
                    digit_vec.push(7);
                    num_str.clear();
                } else if num_str.contains("eight") {
                    digit_vec.push(8);
                    num_str.clear();
                } else if num_str.contains("nine") {
                    digit_vec.push(9);
                    num_str.clear();
                } else {
                }
            }
        }
        let val = digit_vec.first().unwrap() * 10 + digit_vec.last().unwrap();
        total = total + val;
    }

    println!("Solution 2: {}", total);
}
