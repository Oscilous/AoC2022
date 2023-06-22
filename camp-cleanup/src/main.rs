use std::fs;
fn main() {
    let contents: String =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let pairs = split_string(contents);
    compare_ranges(&pairs);
}

fn split_string(s: String) -> Vec<String> {
    let mut split_list: Vec<String> = Vec::new();
    let pairs_string: std::str::Split<&str> = s.split("\n");

    for pair_string in pairs_string {
        split_list.push(pair_string.to_string());
    }
    split_list
}

fn compare_ranges(pairs: &Vec<String>) {
    let mut overlaping_range_amount = 0;

    for pair in pairs {
        let pair = split_string_into_str_tuple(pair, ",");
        let first_sectors = split_string_into_u32_tuple(&pair.0, "-");
        let second_sectors = split_string_into_u32_tuple(&pair.1, "-");

        if (first_sectors.0 >= second_sectors.0) && (first_sectors.1 <= second_sectors.1) {
            println!(
                "Fits {}-{} into {}-{}",
                first_sectors.0, first_sectors.1, second_sectors.0, second_sectors.1
            );
            overlaping_range_amount += 1;
        } else if (first_sectors.0 <= second_sectors.0) && (first_sectors.1 >= second_sectors.1) {
            println!(
                "Into {}-{} goes {}-{}",
                first_sectors.0, first_sectors.1, second_sectors.0, second_sectors.1
            );
            overlaping_range_amount += 1;
        }
    }

    println!("There are {} pairs", overlaping_range_amount);
}

fn split_string_into_str_tuple(input: &str, delimiter: &str) -> (String, String) {
    let mut split_iter = input.split(delimiter);
    let first_part = split_iter.next().unwrap_or("");
    let second_part = split_iter.next().unwrap_or("");
    (first_part.to_string(), second_part.to_string())
}

fn split_string_into_u32_tuple(input: &str, delimiter: &str) -> (u32, u32) {
    let mut split_iter = input.split(delimiter);
    let first_part = split_iter.next().unwrap_or("");
    let second_part = split_iter.next().unwrap_or("");
    (
        first_part.parse::<u32>().unwrap(),
        second_part.parse::<u32>().unwrap(),
    )
}
