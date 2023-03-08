use std::{fs::File, io::{BufReader, Read}};

fn main() {
    let contents = open_file("inputs/input_day_5.txt");
    let input: String;

    match contents {
        Ok(c) => input = c,
        Err(err) => panic!("{}", err)
    }

    println!("{}", first(input.clone()));
    println!("{}", second(input));
}

fn first(input: String) -> i32 {

    let mut map = vec![vec![0; 1000]; 1000];

    for i in input.split('\n') {
        let (cord_1, cord_2) = i.split_once(" -> ").unwrap();
    
        let (x1, y1) = cord_1.split_once(',').unwrap();
        let (x2, y2) = cord_2.split_once(',').unwrap();

        let mut x1 = x1.parse::<usize>().unwrap();
        let mut x2 = x2.parse::<usize>().unwrap();
        let mut y1 = y1.parse::<usize>().unwrap();
        let mut y2 = y2.parse::<usize>().unwrap();

        if x1 == x2 {
            if y2 < y1 {
                (y1, y2) = (y2, y1)
            }
            for j in y1..=y2 {
                map[j][x1] += 1;
            }
        }
        
        if y1 == y2 {
            if x2 < x1 {
                (x1, x2) = (x2, x1)
            }
            for j in x1..=x2 {
                map[y1][j] += 1;
            }
        }
    }

    // print_map(map.clone());

    let mut danger = 0;

    for i in map {
        for j in i {
            if j > 1 {
                danger += 1;
            }
        }
    }

    danger
}

fn second(input: String) -> i32 {

    let mut map = vec![vec![0; 1000]; 1000];

    for i in input.split('\n') {
        let (cord_1, cord_2) = i.split_once(" -> ").unwrap();
    
        let (x1, y1) = cord_1.split_once(',').unwrap();
        let (x2, y2) = cord_2.split_once(',').unwrap();

        let mut x1 = x1.parse::<usize>().unwrap();
        let mut x2 = x2.parse::<usize>().unwrap();
        let mut y1 = y1.parse::<usize>().unwrap();
        let mut y2 = y2.parse::<usize>().unwrap();

        if x1 == x2 {
            if y2 < y1 {
                (y1, y2) = (y2, y1)
            }
            for j in y1..=y2 {
                map[j][x1] += 1;
            }
        }
        
        if y1 == y2 {
            if x2 < x1 {
                (x1, x2) = (x2, x1)
            }
            for j in x1..=x2 {
                map[y1][j] += 1;
            }
        }

        let x_diff = (x1).abs_diff(x2);
        let y_diff = (y1).abs_diff(y2);

        if x_diff == y_diff {
            let direction: &str;

            if x1 > x2 && y1 > y2 || x1 < x2 && y1 < y2{
                direction = "left"
            } else {
                direction = "right"
            }

            let mut x: usize;
            let mut y: usize;

            if x1 > x2 {
                x = x1;
                y = y1;
            } else {
                x = x2;
                y = y2;
            }
            for _ in 0..=x_diff {
                map[y][x] += 1;

                if direction == "left" {
                    if x == 0 || y == 0 {
                        continue;
                    } 
                    x -= 1;
                    y -= 1;
                } else {
                    if x == 0 {
                        continue;
                    }
                    x -= 1;
                    y += 1;
                }
            }
        }
    }

    // print_map(map.clone());

    let mut danger = 0;

    for i in map {
        for j in i {
            if j > 1 {
                danger += 1;
            }
        }
    }

    danger
}

fn print_map(map: Vec<Vec<i32>>) {
    for i in map {
        println!("{:?}", i);
    }
}

fn open_file(file_path: &str) -> std::io::Result<String> {
    let file = File::open(file_path)?;
    let mut buff_reader = BufReader::new(file);
    let mut contents = String::new();
    buff_reader.read_to_string(&mut contents)?;

    Ok(contents)
}
