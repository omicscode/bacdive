use crate::structfile::BacdiveSpeciesJson;
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
pub async fn strain_write(
    path: &str,
    strain: &str,
) -> Result<Vec<BacdiveSpeciesJson>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
    let bacopen = File::open(path).expect("file not found");
    let bacread = BufReader::new(bacopen);
    for i in bacread.lines() {
        let line = i.expect("line not found");
        let idline: Vec<_> = line.split(",").collect::<Vec<_>>();
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
            bacstring.push(line.clone());
        } else {
            continue;
        }
    }

    let mut bacstrain: Vec<BacdiveSpeciesJson> = Vec::new();
    for i in bacstring.iter() {
        if i.trim()
            .split(",")
            .map(|x| x.replace("\"", ""))
            .map(|x| x.replace("#", ""))
            .collect::<Vec<_>>()[2]
            .split(" ")
            .collect::<Vec<_>>()
            .join("")
            .to_string()
            == strain
            && !i
                .trim()
                .split(",")
                .map(|x| x.replace("\"", ""))
                .map(|x| x.replace("#", ""))
                .collect::<Vec<_>>()[2]
                .is_empty()
        {
            bacstrain.push(BacdiveSpeciesJson {
                id: i
                    .trim()
                    .split(",")
                    .map(|x| x.replace("\"", ""))
                    .map(|x| x.replace("#", ""))
                    .collect::<Vec<_>>()[..1]
                    .join(",")
                    .to_string(),
                species: i
                    .trim()
                    .split(",")
                    .map(|x| x.replace("\"", ""))
                    .map(|x| x.replace("#", ""))
                    .collect::<Vec<_>>()[1..2]
                    .join(",")
                    .to_string(),
                strain: i
                    .trim()
                    .split(",")
                    .map(|x| x.replace("\"", ""))
                    .map(|x| x.replace("#", ""))
                    .collect::<Vec<_>>()[2..3]
                    .join(",")
                    .to_string(),
                information: i
                    .trim()
                    .split(",")
                    .map(|x| x.replace("\"", ""))
                    .map(|x| x.replace("#", ""))
                    .collect::<Vec<_>>()[3..]
                    .join(",")
                    .to_string(),
            });
        } else if i
            .trim()
            .split(",")
            .map(|x| x.replace("\"", ""))
            .map(|x| x.replace("#", ""))
            .collect::<Vec<_>>()[2]
            .split(" ")
            .collect::<Vec<_>>()
            .join("")
            .to_string()
            == strain
            && i.trim()
                .split(",")
                .map(|x| x.replace("\"", ""))
                .map(|x| x.replace("#", ""))
                .collect::<Vec<_>>()[2]
                .is_empty()
        {
            bacstrain.push(BacdiveSpeciesJson {
                id: i
                    .trim()
                    .split(",")
                    .map(|x| x.replace("\"", ""))
                    .map(|x| x.replace("#", ""))
                    .collect::<Vec<_>>()[..1]
                    .concat()
                    .to_string(),
                information: i
                    .trim()
                    .split(",")
                    .map(|x| x.replace("\"", ""))
                    .map(|x| x.replace("#", ""))
                    .collect::<Vec<_>>()[1..]
                    .join(",")
                    .to_string(),
                species: "same species".to_string(),
                strain: "same strain".to_string(),
            });
        }
    }
    Ok(bacstrain)
}
