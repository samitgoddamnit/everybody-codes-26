use std::fs;
use std::path::Path;



fn main() {
    let file_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/everybody_codes_e2026_q03_p2.txt"));
    match fs::read_to_string(file_path) {
        Ok(file) =>{
            let mut result: u64 = 0;
            for line in file.lines() {
                let tmp: String = line.to_string();
                let split: Vec<&str> = tmp.split(" ").collect();
                let number = split[1];
                println!("number is {:?}",number);
                let number_bytes = number.as_bytes();
                let mut number_vec = Vec::<u64>::new(); 
                for i in 0..number_bytes.len(){
                    number_vec.push((number_bytes[i] - b'0') as u64);
                }
                // println!("{:?}",number_vec);
                
                let standard = split[0].split("-").collect::<Vec<&str>>()[1];
                println!("standard is {:?}",standard);
                let standard_bytes = standard.as_bytes();
                let mut standard_vec = Vec::<u64>::new();
                for i in 0..standard_bytes.len(){
                    standard_vec.push((standard_bytes[i] - b'0') as u64);
                }
                //reading input correctly!

                let mut denary:u64 = 0;
                let mut standard_index:usize = standard_vec.len()- 1;
                let mut digit_index:usize = number_vec.len() - 1;
                while true{
                    let digit = number_vec[digit_index] as u64;
                    let power = standard_vec[standard_index] as u64;

                    let scale_factor = (power.pow((number_vec.len() - 1 - digit_index) as u32)) as u64;

                    denary += scale_factor * digit;

                    standard_index = ((standard_index as i64 - 1 + standard_vec.len() as i64) % standard_vec.len() as i64) as usize;
                    if digit_index == 0 {
                        break;
                    }
                    digit_index -= 1;
                }

                // println!("{:?}",denary);
                result += denary;
            }
            println!("{:?}",result);
        }
        Err(err) =>{
            println!("Error opening file");
        }
    }
}