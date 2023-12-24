use std::fs;

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

fn main() {
    let file = String::from("./resources/input_02.txt");
    let input = fs::read_to_string(&file).expect(&format!("Unable to open file {}", file.as_str()));

    let mut total = 0;
    for line in input.lines() {
        let mut split_line = line.split_whitespace();
        let mut possible: bool = true;
        // Extract the current game number
        let game_nb = split_line
            .nth(1)
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse::<u32>()
            .unwrap();
        while let Some(v) = split_line.next() {
            // Extract the number of dices
            let value = v.parse::<u32>().unwrap();
            // Extract the dices color
            let dice_color: &str;
            let l_sp = split_line.next().unwrap();
            if let Some(dc) = l_sp.strip_suffix(',') {
                dice_color = dc;
            } else if let Some(dc) = l_sp.strip_suffix(';') {
                dice_color = dc;
            } else {
                dice_color = l_sp;
            }
            // Count the dices by color
            match dice_color {
                "red" => {
                    if value > MAX_RED_CUBES {
                        possible = false;
                    }
                }
                "green" => {
                    if value > MAX_GREEN_CUBES {
                        possible = false;
                    }
                }
                "blue" => {
                    if value > MAX_BLUE_CUBES {
                        possible = false;
                    }
                }
                _ => {}
            }
        }

        if possible {
            total += game_nb;
        }
    }

    println!("Solution 1: {}", total);
}
