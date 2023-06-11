use std::fs;

enum Move {
    Rock,
    Paper,
    Scisors,
}

impl Move {
    fn value(&self) -> i32 {
        match *self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scisors => 3,
        }
    }
}

enum Result {
    Loss,
    Draw,
    Victory,
}

fn main() {
    let contents: String =
        fs::read_to_string("src/test.txt").expect("Should have been able to read the file");
    let mut rounds: Vec<(String, String)> = process_string(contents);
    play_the_game(&rounds);
    convert_to_part_two(&mut rounds);
    play_the_game(&rounds);
}

fn process_string(string: String) -> Vec<(String, String)> {
    let mut rounds = Vec::new();
    let string = string.split("\n");

    for round in string {
        let mut action = round.split(" ");
        let round = (
            action.next().unwrap().to_string(),
            action.next().unwrap().to_string(),
        );
        rounds.push(round);
    }

    rounds
}

fn play_the_game(rounds: &Vec<(String, String)>) {
    let mut points = 0;

    for round in rounds {
        points += play_a_round(round);
    }

    println!("The player gained {} points", points);
}

fn play_a_round((enemy, player): &(String, String)) -> i32 {
    let player = match player.as_str() {
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scisors,
        &_ => panic!("Enemy failed an action"),
    };

    let enemy = match enemy.as_str() {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scisors,
        &_ => panic!("Enemy failed an action"),
    };

    return match player {
        Move::Rock => match enemy {
            Move::Rock => 3 + player.value(),
            Move::Paper => 0 + player.value(),
            Move::Scisors => 6 + player.value(),
        },
        Move::Paper => match enemy {
            Move::Rock => 6 + player.value(),
            Move::Paper => 3 + player.value(),
            Move::Scisors => 0 + player.value(),
        },
        Move::Scisors => match enemy {
            Move::Rock => 0 + player.value(),
            Move::Paper => 6 + player.value(),
            Move::Scisors => 3 + player.value(),
        },
    };
}

fn convert_to_part_two(rounds: &mut Vec<(String, String)>) {
    for mut round in rounds {
        let enemy = match round.0.as_str() {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scisors,
            &_ => panic!("Enemy failed an action"),
        };

        let result = match round.1.as_str() {
            "X" => Result::Loss,
            "Y" => Result::Draw,
            "Z" => Result::Victory,
            &_ => panic!("Enemy failed an action"),
        };

        round.1 = match result {
            Result::Loss => match enemy {
                Move::Rock => String::from("Z"),
                Move::Paper => String::from("X"),
                Move::Scisors => String::from("Y"),
            },
            Result::Draw => match enemy {
                Move::Rock => String::from("X"),
                Move::Paper => String::from("Y"),
                Move::Scisors => String::from("Z"),
            },
            Result::Victory => match enemy {
                Move::Rock => String::from("Y"),
                Move::Paper => String::from("Z"),
                Move::Scisors => String::from("X"),
            },
        };
    }
}
