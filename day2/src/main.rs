use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file_name = "input.txt";

    let file = File::open(file_name).expect("unable to open file");

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let ln = line.expect("unable to read line");

        let result = determine_possiblity(&ln);
    }
}

fn determine_possiblity(input: &str) -> bool {
    let (red, green, blue) = (12, 13, 14);
    let mut colours: HashMap<&str, u32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

    let games = match input.split(':').nth(1) {
        Some(s) => s,
        None => return false,
    };

    let split_games: Vec<&str> = games.trim().split(';').collect();

    for game in split_games {
        let game_res: Vec<&str> = game.split(',').map(|x| x.trim()).collect();
        for amount_colour in game_res {
            let amount_colour = amount_colour.trim();
            let ac: Vec<&str> = amount_colour.split(' ').collect();
            let (amount, colour) = (ac[0], ac[1]);
            println!("amount: {}, colour: {}", amount, colour);

            let amount = match amount.parse::<u32>() {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("{}", e);
                    0
                }
            };

            match colours.get_mut(colour) {
                Some(c) => *c += amount,
                None => (),
            }
        }
    }

    println!("{}", colours["red"]);
    println!("{}", colours["green"]);
    println!("{}", colours["blue"]);

    println!("{:?}", colours);

    if red <= colours["red"] || green <= colours["green"] || blue <= colours["blue"] {
        return false;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let inputs: Vec<&str> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let results: Vec<bool> = vec![true, true, false, false, true];

        println!("{:?}", results);

        for (i, e) in inputs.iter().enumerate() {
            println!("{}", i);
            println!("{}", results[i]);
            let res = determine_possiblity(e);
            assert!(res == results[i]);
        }
    }
}
