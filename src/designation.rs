use crate::structfile::BacdiveSearchSpecies;
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
pub async fn bacdivedesignationsearch(
    bacdive_analyzer: &str,
    designation_header: Option<String>,
) -> Result<Vec<BacdiveSearchSpecies>, Box<dyn Error>> {
    let mut bachold: Vec<String> = Vec::new();
    let bacdiveanalyzer = File::open(bacdive_analyzer).expect("file not found");
    let bacdivereader = BufReader::new(bacdiveanalyzer);
    for i in bacdivereader.lines() {
        let line = i.expect("line not found");
        if line.starts_with("\"") || line.starts_with("ID") || line.is_empty() {
            continue;
        } else if !line.starts_with("\"") || !line.starts_with("ID") {
            bachold.push(line);
        }
    }
    let mut designationsearch: Vec<BacdiveSearchSpecies> = Vec::new();
    if designation_header.is_some() {
        for i in bachold.iter() {
            let linevec: Vec<_> = i
                .split(",")
                .map(|x| x.replace("\"", ""))
                .collect::<Vec<_>>();
            if linevec[2..].join("").to_string().contains(
                &designation_header
                    .clone()
                    .expect("species not found")
                    .to_string(),
            ) {
                let idinsert = linevec[0].to_string();
                let speciesinsert = linevec[1].to_string();
                let information = linevec[2..].join("").to_string();
                designationsearch.push(BacdiveSearchSpecies {
                    id: idinsert.clone(),
                    species: speciesinsert.clone(),
                    speciesinformation: information.clone(),
                });
            }
        }
    }
    Ok::<Vec<BacdiveSearchSpecies>, Box<dyn Error>>(designationsearch)
}
