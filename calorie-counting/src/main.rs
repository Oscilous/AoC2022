use std::fs;

fn main() {
    let contents: String =
        fs::read_to_string("inputs/test.txt").expect("Should have been able to read the file");
    let elves = split_string(contents);
    for calories in &elves {
        println!("{}", calories);
    }
    let most_calories = elves
        .iter()
        .enumerate()
        .max_by(|x, y| x.1.cmp(y.1))
        .unwrap();
    println!(
        "Elf number - {}, is carrying the most calories - {}",
        most_calories.0 + 1,
        most_calories.1
    );
}

fn split_string(s: String) -> Vec<i32> {
    let mut split_list: Vec<i32> = Vec::new();
    let elves_string: std::str::Split<&str> = s.split("\n\n");

    for elf_string in elves_string {
        let items: std::str::Lines = elf_string.lines();
        let mut total_calories: i32 = 0;
        for item in items {
            let calories = match item.parse::<i32>() {
                Ok(number) => number,
                Err(_e) => panic!("Cannot convert to i32"),
            };
            total_calories += calories;
        }
        split_list.push(total_calories);
    }
    split_list
}
