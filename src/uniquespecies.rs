use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
 Author Gaurav Sablok
 SLB Potsdam
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-8-18
 Date 2024-2-23
*/

#[tokio::main]
pub async fn unique_species(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
    let bacopen = File::open(path).expect("file not found");
    let bacread = BufReader::new(bacopen);
    for i in bacread.lines() {
        let line = i.expect("line not found");
        bacstring.push(line);
    }
    let mut uniquespecies: HashSet<String> = HashSet::new();
    for i in bacstring.iter() {
        let idline: Vec<_> = i.split(",").collect::<Vec<_>>();
        if idline[0].chars().nth(0).unwrap() == '1'
            || idline[0].chars().nth(0).unwrap() == '2'
            || idline[0].chars().nth(0).unwrap() == '3'
            || idline[0].chars().nth(0).unwrap() == '4'
            || idline[0].chars().nth(0).unwrap() == '5'
            || idline[0].chars().nth(0).unwrap() == '6'
            || idline[0].chars().nth(0).unwrap() == '7'
            || idline[0].chars().nth(0).unwrap() == '8'
            || idline[0].chars().nth(0).unwrap() == '9'
        {
            uniquespecies.insert(
                idline[1]
                    .replace("\"", "")
                    .split(" ")
                    .collect::<Vec<_>>()
                    .join("-")
                    .to_string(),
            );
        } else {
            continue;
        }
    }

    Ok(uniquespecies)
}
