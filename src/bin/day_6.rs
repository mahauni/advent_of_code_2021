use std::{fs::File, io::{BufReader, Read}};

fn main() {
    let contents = open_file("inputs/input_day_6.txt");
    let input: String;

    match contents {
        Ok(c) => input = c,
        Err(err) => panic!("{}", err)
    }

    let mut arr: [u128; 9] = [0; 9];

    input.split(',').for_each(|i|arr[i.parse::<u128>().unwrap() as usize] += 1);

    println!("{}", get_fishes_after(&arr, 80));
    println!("{}", get_fishes_after(&arr, 256))
}

fn get_fishes_after(arr: &[u128; 9], days: i32) -> u128 {
    let mut fishes = *arr;
    let mut preg;

    for _ in 0..days {
        preg = fishes[0];
        for i in 1..fishes.len() {
            fishes[i - 1] = fishes[i];
        }

        fishes[8] = preg;
        fishes[6] += preg;
    }
    
    return fishes.iter().sum::<u128>();
}

fn open_file(file_path: &str) -> std::io::Result<String> {
    let file = File::open(file_path)?;
    let mut buff_reader = BufReader::new(file);
    let mut contents = String::new();
    buff_reader.read_to_string(&mut contents)?;

    Ok(contents)

}
