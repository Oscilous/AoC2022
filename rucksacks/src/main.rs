use std::fs;

fn main() {
    let contents: String =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let rucksacks = string_to_elves(&contents);
    let groups = string_to_groups(&contents);
    sum_of_priorities_per_elf(&rucksacks);
    sum_of_priorities_per_group(&groups);
}

fn string_to_elves(string: &String) -> Vec<(&str, &str)> {
    let mut rucksacks = Vec::new();
    let string = string.split("\n");

    for rucksack in string {
        let rucksack = rucksack.split_at(rucksack.len() / 2);
        rucksacks.push(rucksack);
    }
    rucksacks
}

fn sum_of_priorities_per_elf(rucksacks: &Vec<(&str, &str)>) {
    let mut sum = 0;

    for rucksack in rucksacks {
        let priority = search_shared_items_per_elf(rucksack);
        sum += priority;
    }

    println!("The sum of the priorities is {}", sum);
}

fn search_shared_items_per_elf(rucksack: &(&str, &str)) -> i32 {
    let mut item_types: Vec<char> = Vec::new();
    let left_pocket_items = rucksack.0.chars();

    for left_pocket_item in left_pocket_items {
        let right_pocket_items = rucksack.1.chars();
        for right_pocket_item in right_pocket_items {
            if right_pocket_item == left_pocket_item {
                if !item_types.contains(&right_pocket_item) {
                    item_types.push(right_pocket_item);
                }
            }
        }
    }
    calculate_the_priority(item_types) as i32
}

fn calculate_the_priority(item_types: Vec<char>) -> u8 {
    for item_type in item_types {
        //Offset in UTF8 is different for uppercase/lowercase letters
        if item_type.is_ascii_uppercase() {
            //Adding a offset of 26, as capitals have different priority
            return item_type.to_string().into_bytes()[0] - 0x40 + 26;
        } else if item_type.is_ascii_lowercase() {
            return item_type.to_string().into_bytes()[0] - 0x60;
        } else {
            panic!("Char is not a letter");
        }
    }
    //If the list is empty
    return 0;
}

fn sum_of_priorities_per_group(rucksacks: &Vec<(&str, &str, &str)>) {
    let mut sum = 0;

    for rucksack in rucksacks {
        let priority = search_shared_item_per_group(rucksack);
        sum += priority;
    }

    println!("The sum of the priorities is {}", sum);
}

fn string_to_groups(string: &String) -> Vec<(&str, &str, &str)> {
    let mut groups = Vec::new();
    let string: Vec<&str> = string.lines().collect();

    for group in string.chunks(3) {
        let group: Vec<&str> = group.to_vec();
        let group = (group[0], group[1], group[2]);
        groups.push(group);
    }

    groups
}

fn search_shared_item_per_group(group: &(&str, &str, &str)) -> i32 {
    let mut item_types: Vec<char> = Vec::new();
    let first_elf_items = group.0.chars();

    for first_elf_item in first_elf_items {
        let second_elf_items = group.1.chars();
        for second_elf_item in second_elf_items {
            let third_elf_items = group.2.chars();
            for third_elf_item in third_elf_items {
                if first_elf_item == second_elf_item && second_elf_item == third_elf_item {
                    if !item_types.contains(&first_elf_item) {
                        item_types.push(first_elf_item);
                    }
                }
            }
        }
    }
    calculate_the_priority(item_types) as i32
}
