use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Record {
    pub id: Option<u64>,
    pub quest: Option<u64>,
    pub part: Option<u64>,
    pub time: Option<u64>,
}

impl Record {}

fn main() {
    let part = std::env::args().nth(1).unwrap_or_default();

    let file_path = PathBuf::from(format!(
        "{}/inputs/04/everybody_codes_e2026_q04_p{}.txt",
        env!("CARGO_MANIFEST_DIR"),
        part
    ));
    match fs::read_to_string(file_path) {
        Ok(file) => match part.as_str() {
            "1" => part1(&file),
            "2" => part2(&file),
            "3" => part3(&file),
            _ => println!("invalid argument"),
        },
        Err(_) => {
            println!("Error opening file");
        }
    }
}

fn part1(file: &str) {
    //database intialisation
    let df: Vec<Record> = initialise_df(file);
    // println!("{:?}", df);

    let mut counts: HashMap<u64, u64> = HashMap::new();
    for record in df {
        let key: Option<u64> = record.id;
        let value: Option<u64> = record.time;
        match key {
            Some(k) => match value {
                Some(v) => {
                    counts.entry(k).and_modify(|item| *item += v).or_insert(v);
                }
                None => {}
            },
            None => {}
        }
    }

    let mut totals: Vec<(&u64, &u64)> = counts.iter().collect();
    totals.sort_by(|a, b| a.1.cmp(b.1));
    println!("{:?}", totals);

    let mut checksum = 0;
    let mut multiplier = 1;

    for item in totals {
        println!("{:?}", item.1);
        checksum += multiplier * item.1;
        multiplier += 1;
    }
    println!("{:?}", checksum);
}

fn part2(file: &str) {
    let df: Vec<Record> = initialise_df(file);

    let mut times: HashMap<u64, u64> = HashMap::new();
    let mut parts: HashMap<u64, u64> = HashMap::new();
    for record in df {
        let key: Option<u64> = record.id;
        let value: Option<u64> = record.time;
        match key {
            Some(k) => {
                parts.entry(k).and_modify(|item| *item += 1).or_insert(1);
                match value {
                    Some(v) => {
                        times.entry(k).and_modify(|item| *item += v).or_insert(v);
                    }
                    None => {}
                }
            }
            None => {}
        }
    }

    let mut id_data: Vec<(u64, u64, u64)> = Vec::new();
    for (&id, &time) in &times {
        if let Some(&p) = parts.get(&id) {
            id_data.push((id, time, p));
        }
    }

    id_data.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| a.1.cmp(&b.1)));

    println!("{:?}", id_data);

    let mut checksum = 0;
    let mut multiplier = 1;

    for item in id_data {
        println!("{:?}", item.1);
        checksum += multiplier * item.1;
        multiplier += 1;
    }
    println!("{:?}", checksum);
}

fn part3(file: &str) {
    let df: Vec<Record> = initialise_df(file);
}

fn initialise_df(file: &str) -> Vec<Record> {
    let mut df: Vec<Record> = Vec::new();
    for line in file.lines() {
        let mut line_record = Record::default();
        println!("{:?}", line);
        let parts: Vec<&str> = line.split(" ").collect();
        for part in parts {
            let key_value_pair: Vec<&str> = part.split("=").collect();
            let key = key_value_pair[0];
            let value = key_value_pair[1];
            match key {
                "id" => line_record.id = Some(value.parse::<u64>().unwrap()),
                "quest" => line_record.quest = Some(value.parse::<u64>().unwrap()),
                "part" => line_record.part = Some(value.parse::<u64>().unwrap()),
                "time" => line_record.time = Some(value.parse::<u64>().unwrap()),
                _ => println!("error found parsing record"),
            }
        }
        df.push(line_record);
    }
    df
}
