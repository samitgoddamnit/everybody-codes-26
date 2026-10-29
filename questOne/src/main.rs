use std::error::Error;
use std::fs;
use std::path::Path;

fn main() {
    let file_path = Path::new(
        r"C:\Users\joshu\Documents\Project Stuff\everybodyCodes\questOne\everybody_codes_e2026_q01_p3.txt",
    );

    let mut total: i64 = 0;
    match fs::read_to_string(file_path) {
        Ok(file) => {
            // println!("{:?}", file);
            let trimmed: Vec<&str> = file.split(",").collect();
            let mut squared: i64 = 9;
            let mut base: i64 = 3;

            for item in trimmed {
                match item.parse::<i64>() {
                    Ok(number) => {
                        while number > squared - 1 {
                            base += 2;
                            squared = base * base;
                        }
                        let difference = squared - 1 - number;
                        total += difference;
                    }
                    Err(e) => {
                        println!("kill yourself");
                    }
                }
            }
        }
        Err(err) => {
            println!("Error opening file");
        }
    }
    println!("{:?}", total);
}
