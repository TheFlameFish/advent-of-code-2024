use std::{fs::File, io::Read, path::Path};

fn main() {
    // Create a path to the desired file
    let path = Path::new("../../inputs/day1.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("{} has been read succesfully.", display),
    }

    let (left, right) = seperate(s);

    println!("Similarity of {}.", rate_similarity(&left, &right));
}

fn rate_similarity(v1: &Vec<i128>, v2: &Vec<i128>) -> i128 {
    let mut score: i128 = 0;
    for (i, o) in v1.iter().enumerate() {
        let occourences = find_occourences(o, v2);

        score += o * occourences;
    }

    score
}

fn find_occourences(of: &i128, inside: &Vec<i128>) -> i128 {
    let mut quantity: i128 = 0;

    for o in inside.iter() {
        if o == of {
            quantity += 1;
        }
    }

    quantity
}

fn seperate(s: String) -> (Vec<i128>, Vec<i128>) {
    let mut left: Vec<i128> = Vec::new();
    let mut right: Vec<i128> = Vec::new();

    // Replace series of 4 spaces with tab
    let mut new_s = String::from("");
    let mut spaces: i8 = 0;
    for o in s.chars() {
        if o == ' ' {
            spaces += 1;
            if spaces == 3 {
                new_s.push('\t');
                spaces = 0;
            }
        } else {
            new_s.push(o);
        }
    }

    let mut current_str: String = "".to_string();
    for o in new_s.chars() {
        if o == '\t' {
            let parsed_id: i128 = match current_str.parse() {
                Ok(id) => id,
                Err(error) => panic!("Invalid id: {current_str}\nError: {error}")
            };

            left.push(parsed_id);
            current_str = "".to_string();
            continue;
        } else if o == 0xA as char {
            let parsed_id: i128 = match current_str.parse() {
                Ok(id) => id,
                Err(error) => panic!("Invalid id: {current_str}\nError: {error}")
            };

            right.push(parsed_id);
            current_str = "".to_string();
            continue;
        }

        current_str.push_str(&o.to_string());
    }

    (left, right)
}