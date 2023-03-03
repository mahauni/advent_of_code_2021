use std::{fs::File, io::{BufReader, Read}};

fn main() {
    let contents = open_file("inputs/input_day_2.txt");

    let input: String;

    match contents {
        Ok(c) => input = c,
        Err(err) => panic!("{}", err)
    } 

    println!("{}", first(input.clone()));

    println!("{}", second(input));
}

fn first(input: String) -> i32 {
    let mut depth = 0;
    let mut horizontal = 0;

    for i in input.split('\n') {
        let (mut direction, mut val): (&str, &str) = ("", "");

        match i.split_once(' ') {
            Some(s) => (direction, val) = (s.0, s.1),
            None => ()
        }

        match direction {
            "forward" => horizontal += val.parse::<i32>().unwrap(),
            "down" => depth += val.parse::<i32>().unwrap(),
            "up" => depth -= val.parse::<i32>().unwrap(),
            _ => ()
        }
    }

    horizontal * depth
}

fn second(input: String) -> i32{
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for i in input.split('\n') {
        let (mut direction, mut val): (&str, &str) = ("", "");

        match i.split_once(' ') {
            Some(s) => (direction, val) = (s.0, s.1),
            None => ()
        }

        match direction {
            "forward" => {    
                horizontal += val.parse::<i32>().unwrap();
                depth += aim * val.parse::<i32>().unwrap()
            },
            "down" => {
                aim += val.parse::<i32>().unwrap()
            },
            "up" => {
                aim -= val.parse::<i32>().unwrap()
            },
            _ => ()
        }
    }

    horizontal * depth

}

fn open_file(file_path: &str) -> std::io::Result<String> {
    let file = File::open(file_path)?;
    let mut buff_reader = BufReader::new(file);
    let mut contents = String::new();
    buff_reader.read_to_string(&mut contents)?;

    Ok(contents)
}
