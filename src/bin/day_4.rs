use std::{
    fs::File,
    io::{BufReader, Read},
};
use std::{collections::HashSet, str::Lines};

fn main() {
    let content = open_file("inputs/input_day_4.txt");
    let input: String;

    match content {
        Ok(c) => input = c,
        Err(err) => panic!("{}", err),
    }

    println!("{}", first(input));
    println!("{}", second());
}

fn horizontal_check(card: &String, range: &[&str]) -> bool {
    let mut count = 0;
    for i in card.split('\n') {
        for j in i.split_whitespace() {
            if range.contains(&j) {
                count += 1;
            } else {
                break;
            }
        }

        if count == 5 {
            return true
        }

        count = 0;
    }

    false
}

fn vertical_check(arr: &[&str; 5], range: &[&str]) -> bool {
    let mut count = 0;
    for i in arr {
        if range.contains(&i) {
            count += 1;
        } else {
            break;
        }
    }
    if count == 5 {
        return true;
    }

    false
}

fn diagonal_check(range: &[&str], arr1: &[&str; 5], arr2: &[&str; 5], arr3: &[&str; 5], arr4: &[&str; 5], arr5: &[&str; 5]) -> bool {
    // diagonal check
    let mut count = 0;
    for i in 0..5 {
        match i {
            0 => {
                if range.contains(&arr1[0]) {
                    count += 1
                } else {
                    break
                }
            },
            1 => {
                if range.contains(&arr2[1]) {
                    count += 1
                } else {
                    break
                }
            },
            2 => {
                if range.contains(&arr3[2]) {
                    count += 1
                } else {
                    break
                }
            },
            3 => {
                if range.contains(&arr4[3]) {
                    count += 1
                } else {
                    break
                }
            },
            4 => {
                if range.contains(&arr5[4]) {
                    count += 1
                } else {
                    break
                }
            },
            _ => ()
        }
    }

    if count == 5{
        return true;
    }

    count = 0;
    for i in 0..5 {
        match i {
            0 => {
                if range.contains(&arr1[4]) {
                    count += 1
                } else {
                    break
                }
            },
            1 => {
                if range.contains(&arr2[3]) {
                    count += 1
                } else {
                    break
                }
            },
            2 => {
                if range.contains(&arr3[2]) {
                    count += 1
                } else {
                    break
                }
            },
            3 => {
                if range.contains(&arr4[1]) {
                    count += 1
                } else {
                    break
                }
            },
            4 => {
                if range.contains(&arr5[0]) {
                    count += 1
                } else {
                    break
                }
            },
            _ => ()
        }
    }

    if count == 5 {
        return true;
    }

    false
}

fn find_bingo(card: String, balls: &Vec<&str>, n: i32, diagonal: bool) -> bool {
    let (range, _) = balls.split_at(n as usize);

    // vertical check
    let mut arr1: [&str; 5] = ["", "", "", "", ""];
    let mut arr2: [&str; 5] = ["", "", "", "", ""];
    let mut arr3: [&str; 5] = ["", "", "", "", ""];
    let mut arr4: [&str; 5] = ["", "", "", "", ""];
    let mut arr5: [&str; 5] = ["", "", "", "", ""];
    let mut line = 0;

    for i in card.split('\n') {
        for (i, v) in i.split_whitespace().into_iter().enumerate() {
           match i {
                0 => arr1[line] = v,
                1 => arr2[line] = v,
                2 => arr3[line] = v,
                3 => arr4[line] = v,
                4 => arr5[line] = v,
                _ => ()
            }
        }

        line += 1;
    }    

    if vertical_check(&arr1, &range) {
        return true;
    }

    if vertical_check(&arr2, &range) {
        return true;
    }

    if vertical_check(&arr3, &range) {
        return true;
    }

    if vertical_check(&arr4, &range) {
        return true;
    }

    if vertical_check(&arr5, &range) {
        return true;
    }

    if diagonal_check(&range, &arr1, &arr2, &arr3, &arr4, &arr5) && diagonal {
        return true;
    }

    if horizontal_check(&card, &range) {
        return true
    }
    

    false
}

fn first(input: String) -> i32 {
    let mut card: String = String::new();
    let mut n = 5;
    let mut found = false;
    let mut s: String = String::new();

    let (bingo, input) = input.split_once('\n').unwrap();

    let bingo: Vec<&str> = bingo.split(',').collect(); 
    
    while found != true {
        for i in input.split('\n') {
            if !i.is_empty() {
                let mut line: String = i.to_string();
                line.push('\n');
                card.push_str(line.as_str());
            } else {
                found = find_bingo(card.clone(), &bingo, n, false); 
                if found {
                    s = card.clone();
                    break;
                }
                card.clear();
            }
        }

        n += 1;

    }

    let (range, _) = bingo.split_at((n-1) as usize);
    let mut unmarked_nums = 0;

    for i in s.split('\n') {
        for j in i.split_whitespace() {
            if !range.contains(&j) {
                unmarked_nums += j.parse::<i32>().unwrap();
            }
        }
    }

    unmarked_nums * range.last().unwrap().parse::<i32>().unwrap()
}


// Second exercise taken from tjdevries 
// Will try latter to make one myself
struct Board {
    /// List of sets of winning combinations.
    sets: Vec<HashSet<i32>>,
}

impl Board {
    fn new(lines: &mut Lines) -> Option<Board> {
        let mut sets: Vec<HashSet<i32>> = Vec::new();

        // Read empty line
        let empty = lines.next()?;
        if empty != "" {
            panic!("Dude, surely you messed something up");
        }

        // [
        //  [7, 13, 22, 2, 1],
        //  [3, 5, 10, 9, 8],
        //  ...
        // ]
        let rows: Vec<Vec<i32>> = lines
            .take(5)
            .map(|l| l.split_whitespace().map(|m| m.parse().unwrap()).collect())
            .collect();

        // Column-wise
        for col in 0..5 {
            let mut set = HashSet::new();
            for row in 0..5 {
                set.insert(rows[row][col]);
            }

            sets.push(set);
        }

        // Row-wise
        for row in rows {
            sets.push(HashSet::from_iter(row));
        }

        // No diagonals in this mode

        Some(Board { sets })
    }

    fn turn(&mut self, m: i32) -> bool {
        let mut complete = false;
        for set in self.sets.iter_mut() {
            if set.remove(&m) {
                complete |= set.is_empty();
            }
        }

        complete
    }

    fn remaining_sum(&self) -> i32 {
        HashSet::<&i32>::from_iter(self.sets.iter().flatten())
            .into_iter()
            .sum()

        // // Alternatively:
        // let mut remaining = HashSet::new();
        //
        // for s in self.sets.iter() {
        //     remaining.extend(s);
        // }
        //
        // remaining.iter().sum()
    }
}

fn second() -> i32 {
    let mut lines = include_str!("../../inputs/input_day_4.txt").lines();
    let moves: Vec<i32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|m| m.parse::<i32>().unwrap())
        .collect();

    let mut boards = Vec::new();
    while let Some(board) = Board::new(&mut lines) {
        boards.push(board)
    }
    
    let mut last_result = 0;
    for m in &moves {
        let mut to_remove = Vec::new();
        for (idx, board) in boards.iter_mut().enumerate() {
            if board.turn(*m) {
                last_result = m * board.remaining_sum();
                to_remove.push(idx);
            }
        }
        // Remove boards that are complete.
        // Iterate back-to-front for indexes to be correct
        for idx in to_remove.iter().rev() {
            boards.remove(*idx);
        }
    }

    last_result
}

fn open_file(file_path: &str) -> std::io::Result<String> {
    let file = File::open(file_path)?;
    let mut buff_reader = BufReader::new(file);
    let mut contents = String::new();
    buff_reader.read_to_string(&mut contents)?;

    Ok(contents)
}
