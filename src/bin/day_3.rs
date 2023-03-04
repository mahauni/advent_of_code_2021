use std::{fs::File, io::{BufReader, Read}, collections::HashMap};

fn main() {
    let contents = open_file("inputs/input_day_3.txt");
    let input: String;

    match contents {
        Ok(c) => input = c,
        Err(err) => panic!("{}", err)
    }

    println!("{}", first(input.clone()));
    
    println!("{}", second(input));
}

fn first(input: String) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut digit = 1;

    for i in input.split('\n') {
        for j in i.chars() {
            match map.get(&digit) {
                Some(n) => {
                    if j == '0' {
                        map.insert(digit, n - 1);
                    } else {
                        map.insert(digit, n + 1);
                    }
                },
                None => {
                    if j == '0' {
                        map.insert(digit, 0);
                    } else {
                        map.insert(digit, 1);
                    }
                }
            }    
        
            digit += 1;
        }
    
        digit = 1;
    }

    let mut gama: String = String::new();
    let mut epsilon: String = String::new();

    for i in 1..=map.len() {
        match map.get(&(i as i32)) {
            Some(n) => {
                if n > &0 {
                    gama.push('1');
                    epsilon.push('0');
                } else {
                    gama.push('0');
                    epsilon.push('1');
                }
            },
            None => ()
        }
    }

    let gama = i32::from_str_radix(gama.as_str(), 2).expect("A binary string");
    let epsilon = i32::from_str_radix(epsilon.as_str(), 2).expect("A binary string");

    gama * epsilon
}

fn second(input: String) -> i32 {
    let mut arr1: Vec<String> = Vec::new();
    let mut arr2: Vec<String> = Vec::new();

    for i in input.split('\n') {
        match i.chars().nth(0) {
            Some(n) => {
                if n == '0' {
                    arr1.push(i.to_string());
                } else {
                    arr2.push(i.to_string());
                }
            },
            None => ()
        }
    }

    let mut max: Vec<String>;
    let mut min: Vec<String>;

    if arr1.len() > arr2.len() {
        min = arr2;
        max = arr1;
    } else {
        min = arr1;
        max = arr2;
    }

    let mut digit = 1;
    let mut total = 0;

    while &max.len() > &(1 as usize) {
        for ele in max.clone().into_iter() {
            if ele.chars().nth(digit).unwrap() == '1' {
                total += 1;
            } else {
                total -= 1;
            }
        }

        if total >= 0 {
            (max, _) = max.clone().into_iter().partition(|e| e.chars().nth(digit).unwrap() == '1');
        } else {
            (max, _) = max.clone().into_iter().partition(|e| e.chars().nth(digit).unwrap() == '0');
        }
   
        digit += 1;
        total = 0;
    }

    digit = 1;
    total = 0;

    while &min.len() > &(1 as usize) {
        for ele in min.clone().into_iter() {
            if ele.chars().nth(digit).unwrap() == '1' {
                total += 1;
            } else {
                total -= 1;
            }
        }

        if total >= 0 {
            (min, _) = min.clone().into_iter().partition(|e| e.chars().nth(digit).unwrap() == '0');
        } else {
            (min, _) = min.clone().into_iter().partition(|e| e.chars().nth(digit).unwrap() == '1');
        }
   
        digit += 1;
        total = 0;
    }

    let o2 = i32::from_str_radix(max.first().unwrap().as_str(), 2).expect("A binary string");
    let co2 = i32::from_str_radix(min.first().unwrap().as_str(), 2).expect("A binary string");

    o2 * co2
}

fn open_file(file_path: &str) -> std::io::Result<String>{
    let file = File::open(file_path)?;
    let mut buff_reader = BufReader::new(file);
    let mut contents = String::new();
    buff_reader.read_to_string(&mut contents)?;

    Ok(contents)
}
