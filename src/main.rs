use std::env;
use std::fs::read_to_string;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
#[allow(dead_code,non_snake_case)]
struct Record {
    FNAME: String,
    LNAME: String,
    AGE: Option<u8>,
    ADDRESS: String,
    CITY: String,
    STATE: String,
    COUNTRY: String,
}

// remove ### comments from the csv file
fn remove_comments(line: &str) -> &str {
    let line_sans_comments_array: Vec<&str> = line.split("###").collect();
    if line_sans_comments_array.len() < 1 {
        return ""
    }
    line_sans_comments_array[0]
}

fn convert_to_records(filename: String) -> Vec<Record> {
    let mut list_of_records: Vec<Record> = vec!();

    let contents = match read_to_string(filename) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
    
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut clean_lines = String::from("");

    for l in lines {

        // afterwards, attempt to clean each line
        let current_line_elements: Vec<&str> = remove_comments(l).split(',').collect();
        let mut clean_line = String::from("");

        for element in current_line_elements {
            clean_line = [
                clean_line,
                element.trim().to_string(),
                ",".to_string(),
            ].concat();
        }

        clean_lines = [
            clean_lines,
            "\n".to_string(),
            clean_line.trim_end_matches(',').to_string(),
        ].concat();
    }

    let mut rdr = csv::Reader::from_reader(clean_lines.as_bytes());

    let mut num_of_errors = 0;
    for res in rdr.deserialize::<Record>() {
        let record: Record = match res {
            Ok(r) => r,
            Err(_) => {
                num_of_errors+=1;
                continue
            },
        };
        list_of_records.push(record);
    }

    if num_of_errors > 0 {
        println!("Number of broken lines that were skipped: {}", num_of_errors);
    }

    list_of_records
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let flag: &String;
    let filename: &String;

    match args.len() {
        3 => {
            flag = &args[1];
            filename = &args[2];
        },
        _ => {
            println!("Usage: ./rust-text-parser --input /path/to/filename.txt");
            return;
        }
    }

    if flag.is_empty() || filename.is_empty() || flag != "--input" {
        return;
    }

    let records = convert_to_records(filename.to_string());

    println!("Records:\n{:#?}",records);
}
