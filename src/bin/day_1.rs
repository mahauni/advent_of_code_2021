use std::{fs::File, io::{BufReader, Read}};

fn main() {
    let contents = open_file("inputs/input_day_1.txt");
    
    let input: String;

    match contents {
        Ok(c) => input = c,
        Err(err) => panic!("{}", err)
    }

    println!("{}", first(input.clone()));

    println!("{}", second(input));
}

fn first(input: String) -> i32 {
    let mut prev = -1;
    let mut total = 0; 
    
    for i in input.split('\n') {
        if prev == -1 {
            prev = i.parse::<i32>().unwrap();
            continue;
        }
    
        if prev < i.parse::<i32>().unwrap() {
            total += 1;
        }

        prev = i.parse::<i32>().unwrap();

    }

    total
}

fn second(input: String) -> i32 {
    let mut window: [i32; 3] = [0, 0, 0];
    let mut iter: i32  = 0;
    let mut prev = -1;
    let mut total = 0;


    for i in input.split('\n') {
        if iter < 3 {
            window[iter as usize] = i.parse::<i32>().unwrap();            
            iter += 1;
            continue;
        }

        if prev < 0 {
            prev = window.iter().sum();
            (window[0], window[1], window[2]) = (window[1], window[2], i.parse::<i32>().unwrap());
            continue;
        }

        let sum: i32 = window.iter().sum();

        if sum > prev {
            total += 1;
        }

        prev = sum;

        (window[0], window[1], window[2]) = (window[1], window[2], i.parse::<i32>().unwrap());

    }

    if window.iter().sum::<i32>() > prev {
        return total + 1
    }

    total
}

fn open_file(file_path: &str) -> std::io::Result<String> {
   let file = File::open(file_path)?;
   let mut buff_reader = BufReader::new(file);
   let mut contents = String::new();
   buff_reader.read_to_string(&mut contents)?;

    Ok(contents)
}
