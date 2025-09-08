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
pub async fn strainheader(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut strainheader: HashSet<String> = HashSet::new();
    for i in fileread.lines() {
        let line = i.expect("file not found");
        if line.starts_with("\"") || line.starts_with("ID") || line.is_empty() {
            continue;
        } else if !line.starts_with("\"") || !line.starts_with("\"") || !line.is_empty() {
            let header = &line
                .split(",")
                .map(|x| x.replace("\"", ""))
                .collect::<Vec<_>>()[4];
            if header.clone() == "0" || header.clone() == "1" {
                strainheader.insert(header.to_string());
            }
        }
    }
    Ok(strainheader)
}
