use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // take some input file
    // parse it

    let file_name = "input.txt";
    let file = File::open(file_name).expect("unable to read file");

    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let l = line.expect("unable to read line");
        let res = calculate_calibration(&l[0..]);
        sum += res;
    }

    println!("Sum: {}", sum);
}

fn calculate_calibration(input: &str) -> u32 {
    let mut first = 0;
    let mut second = 0;

    for c in input.chars().into_iter() {
        if !c.is_digit(10) {
            continue;
        }

        let val = c.to_digit(10).expect("unable to convert to digit");

        if first == 0 {
            first = val;
        }

        second = val
    }

    return first * 10 + second;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_part1() {
        let tests: Vec<&str> = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let expected: Vec<u32> = vec![12, 38, 15, 77];

        for (i, t) in tests.iter().enumerate() {
            assert_eq!(calculate_calibration(t), expected[i]);
        }
    }

    #[test]
    fn part2() {
        let inputs: Vec<&str> = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        let expected: Vec<u32> = vec![29, 83, 13, 24, 42, 14, 76];

        for (i, t) in inputs.iter().enumerate() {
            assert_eq!(calculate_calibration(t), expected[i]);
        }
    }
}
