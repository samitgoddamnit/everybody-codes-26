use std::fs;
use std::path::Path;

fn main() {
    let file_path = Path::new(
        concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/everybody_codes_e2026_q02_p1.txt"),
    );

    //60 and 62 are the less than and greater than symbols
    match fs::read_to_string(file_path) {
        Ok(file) => {
            let mut result: u64 = 0;
            for line in file.lines() {
                let tmp: String = line.to_string();
                let split: Vec<&str> = tmp.split(" ").collect();
                let mut numbers: Vec<(u32, u32)> = Vec::new();

                for i in 1..6 {
                    let mut pair: (u32, u32) = (i, 0);
                    numbers.push(pair);
                }

                for string in split {
                    let bytes = string.as_bytes();
                    if bytes[1] == b'>' {
                        let target: u8 = bytes[0] - b'1';
                        numbers[target as usize].1 += 1;
                    } else {
                        let target: u8 = bytes[2] - b'1';
                        numbers[target as usize].1 += 1;
                    }
                }
                numbers.sort_by(|a, b| b.1.cmp(&a.1));

                println!("{:?}", numbers);
                let mut checksum_str: String = String::new();
                for pair in numbers {
                    checksum_str.push_str(&pair.0.to_string());
                }
                println!("{:?}", checksum_str);
                match checksum_str.parse::<u64>() {
                    Ok(number) => {
                        result += number;
                    }
                    Err(err) => {
                        println!("Error parsing int");
                    }
                }
            }
            println!("{:?}", result);
        }
        Err(err) => {
            println!("Error opening file");
        }
    }
}
