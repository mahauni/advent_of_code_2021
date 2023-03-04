use std::{fs::File, io::{BufReader, Read}};

fn main() {
    let content = open_file("inputs/input_day_4.txt");
    let input: String;

    match content {
        Ok(c) => input = c,
        Err(err) => panic!("{}", err)
    }

    println!("{}", input);
}


fn open_file(file_path: &str) -> std::io::Result<String> {
    let file = File::open(file_path)?;
    let mut buff_reader = BufReader::new(file);
    let mut contents = String::new();
    buff_reader.read_to_string(&mut contents)?;

    Ok(contents)
}
