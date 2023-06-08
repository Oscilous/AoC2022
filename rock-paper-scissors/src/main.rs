//REDO MAKE THE CHOICES INTO ENUMS, as they can only be rock/paper/scissa

use std::fs;

fn main() {
    let contents: String =
        fs::read_to_string("src/test.txt").expect("Should have been able to read the file");
    let rounds: Vec<(String, String)> = process_string(contents);
    play_the_game(rounds);
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

fn play_the_game(rounds: Vec<(String, String)>) {
    let mut points = 0;

    for round in rounds {
        points += play_a_round(round);
    }

    println!("The player gained {} points", points);
}

fn play_a_round((enemy, player): (String, String)) -> i32 {
    let player = match player.as_str() {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        &_ => panic!("Enemy failed an action"),
    };

    let enemy = match enemy.as_str() {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        &_ => panic!("Enemy failed an action"),
    };

    if player == enemy {
        return 3 + player;
    }

    return match player {
        1 => match enemy {
            2 => 0 + player,
            3 => 6 + player,
        },
        2 => match enemy {
            1 => 6 + player,
            3 => 0 + player,
        },
        3 => match enemy {
            1 => 0 + player,
            2 => 6 + player,
        },
    };
}
