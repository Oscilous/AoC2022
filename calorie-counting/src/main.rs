use std::fs;

fn main() {
    let contents =
        fs::read_to_string("inputs/test.txt").expect("Should have been able to read the file");
    let mut elfs = split_string(contents);
    //print!("{}", contents);
}

fn split_string(s: String) {
    // -> Vec<String> {
    //let mut split_list: Vec<String>;
    let mut elves_string = s.split("\n\n");

    for elf_string in elves_string {
        let items = elf_string.lines();
        let mut total_calories: i32 = 0;
        for item in items {
            let calories = match item.parse::<i32>() {
                Ok(number) => number,
                Err(e) => panic!("Cannot convert to i32"),
            };
            total_calories += calories;
        }
        println!("{total_calories}");
    }
    //split_list
}
