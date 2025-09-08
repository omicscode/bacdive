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
pub async fn species(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let mut specieshash: HashSet<String> = HashSet::new();
    for i in fileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with("\"") || line.starts_with("ID") || line.is_empty() {
            continue;
        } else if !line.starts_with("\"") || !line.starts_with("ID") || !line.is_empty() {
            specieshash.insert(
                line.split(",")
                    .map(|x| x.replace("\"", ""))
                    .collect::<Vec<_>>()[1]
                    .split(",")
                    .collect::<Vec<_>>()
                    .join("-")
                    .to_string(),
            );
        }
    }
    Ok(specieshash)
}
