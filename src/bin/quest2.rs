use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let file_path = Path::new(
        concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/everybody_codes_e2026_q02_p3.txt"),
    );

    //60 and 62 are the less than and greater than symbols
    match fs::read_to_string(file_path) {
        Ok(file) => {
            let mut result: u64 = 0;
            for line in file.lines() {
                let tmp: String = line.to_string();
                let split: Vec<&str> = tmp.split(" ").collect();
                let mut graph: HashMap<u32,Vec<u32>> = HashMap::new();
                for string in split {
                    let bytes = string.as_bytes();
                    if bytes[1] == b'>' {
                        graph.entry((bytes[2] - b'0') as u32).or_default().push((bytes[0] - b'0') as u32);
                    } else {
                        graph.entry((bytes[0] - b'0') as u32).or_default().push((bytes[2] - b'0') as u32);
                    }
                }

                let mut path:Vec<u32> = Vec::new();
                let mut best:Vec<u32> = Vec::new();

                for i in 1..10 {
                    dfs(i,&mut path,&mut best,&graph)
                }
                println!("{:?}",best);

                let mut iter = best.iter();
                let mut counter = 0;
                let mut value = 0;
                while let Some(x) = iter.next(){
                    value += x * (10_u32.pow(counter));
                    counter += 1;
                }
                println!("{:?}",value);
                result += value as u64;
            }
            println!("{:?}",result);
        }
        Err(err) => {
            println!("Error opening file");
        }
    }
}

fn dfs(current: u32, path: &mut Vec<u32>, best: &mut Vec<u32>, graph: &HashMap<u32,Vec<u32>>){
    // println!("{:?}",current);
    path.push(current);

    if path.len() > best.len(){
        best.clone_from(path);
    }

    if let Some(options) = graph.get(&current) {
        for &next in options{
            dfs(next,path,best,graph);
        }
    }
    path.pop();
}
