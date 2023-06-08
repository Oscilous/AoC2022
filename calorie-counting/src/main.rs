use std::fs;

fn main() {
    let contents: String =
        fs::read_to_string("src/test.txt").expect("Should have been able to read the file");
    let mut elves: Vec<i32> = split_string(contents);
    find_max_calories(&elves);
    find_top3_calories(&mut elves);
}

fn split_string(s: String) -> Vec<i32> {
    let mut split_list: Vec<i32> = Vec::new();
    let elves_string: std::str::Split<&str> = s.split("\n\n");

    for elf_string in elves_string {
        let items: std::str::Lines = elf_string.lines();
        let mut total_calories: i32 = 0;
        for item in items {
            let calories: i32 = item.parse::<i32>().expect("Cannot convert to i32");
            total_calories += calories;
        }
        split_list.push(total_calories);
    }
    split_list
}

fn find_max_calories(elves: &Vec<i32>) {
    let most_calories: (usize, &i32) = elves
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

fn find_top3_calories(elves: &mut Vec<i32>) {
    elves.sort_by(|x, y| y.cmp(x));
    println!(
        "The sum of the top 3 are {}",
        elves[0] + elves[1] + elves[2]
    );
}
