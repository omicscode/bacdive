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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct BacdiveSearchSpecies {
    pub id: String,
    pub species: String,
    pub speciesinformation: String,
}

#[tokio::main]
pub async fn bacdiveidsearch(
    bacdive_analyzer: &str,
    bacdiveid: Option<String>,
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
    let mut idsearch: Vec<BacdiveSearchSpecies> = Vec::new();
    if bacdiveid.is_some() {
        for i in bachold.iter() {
            let linevec: Vec<_> = i
                .split(",")
                .map(|x| x.replace("\"", ""))
                .collect::<Vec<_>>();
            if linevec[0].to_string() == bacdiveid.clone().expect("id not found").to_string() {
                let idinsert = linevec[0].to_string();
                let speciesinsert = linevec[1].to_string();
                let information = linevec[2..].join("").to_string();
                idsearch.push(BacdiveSearchSpecies {
                    id: idinsert.clone(),
                    species: speciesinsert.clone(),
                    speciesinformation: information.clone(),
                });
            }
        }
    }
    Ok::<Vec<BacdiveSearchSpecies>, Box<dyn Error>>(idsearch)
}
