mod args;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
use serde::Serialize;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
*Author Gaurav Sablok
*Universitat Potsdam and SLB Potsdam
*Date 2024-1-24

bacdive analyzer- analyze the microbial genotypes using the
rust-bacdive standalone. It will prepare all the files for the
json API for the bacdive as well as the bacdive for the sql
insertion.

*/

fn main() {
    let bacdiveargs = CommandParse::parse();
    match &bacdiveargs.command {
        Commands::Id { bacdive, id } => {
            let commandoutput = id_write(bacdive, id).unwrap();
            println!("The ids are: {:?}", commandoutput);
        }
        Commands::Species { bacdive, species } => {
            let commandoutput = species_write(bacdive, species).unwrap();
            println!(
                "The species and the associated information are: {:?}",
                commandoutput
            );
        }
        Commands::Strain { bacdive, strain } => {
            let commandoutput = strain_write(bacdive, strain).unwrap();
            println!(
                "The strain specific information are as follows:{:?}",
                commandoutput
            );
        }
        Commands::IdList { bacdive } => {
            let commandoutput = unique_id(bacdive).unwrap();
            println!("The category2 searches are: {:?}", commandoutput);
        }
        Commands::SpeciesList { bacdive } => {
            let commandoutput = unique_species(bacdive).unwrap();
            println!("The category2 searches are: {:?}", commandoutput);
        }
        Commands::Strainlist { bacdive } => {
            let commandoutput = unique_strain(bacdive).unwrap();
            println!("The category2 searches are: {:?}", commandoutput);
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub struct BacdiveFilter {
    pub id: String,
    pub species: String,
    pub strain: String,
    pub information: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
pub struct BacdiveSpeciesJson {
    pub id: String,
    pub species: String,
    pub strain: String,
    pub information: String,
}

fn unique_id(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
    let bacopen = File::open(path).expect("file not found");
    let bacread = BufReader::new(bacopen);
    for i in bacread.lines() {
        let line = i.expect("line not found");
        bacstring.push(line);
    }
    let mut uniqueid: HashSet<String> = HashSet::new();
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
            uniqueid.insert(idline[0].to_string());
        } else {
            continue;
        }
    }

    Ok(uniqueid)
}

fn unique_strain(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut bacstring: Vec<String> = Vec::new();
    let bacopen = File::open(path).expect("file not found");
    let bacread = BufReader::new(bacopen);
    for i in bacread.lines() {
        let line = i.expect("line not found");
        bacstring.push(line);
    }
    let mut uniquestrain: HashSet<String> = HashSet::new();
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
            uniquestrain.insert(
                idline[2]
                    .replace("\"", "")
                    .split(" ")
                    .collect::<Vec<_>>()
                    .join("")
                    .to_string(),
            );
        } else {
            continue;
        }
    }

    Ok(uniquestrain)
}

fn unique_species(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
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

fn id_write(path: &str, id: &str) -> Result<Vec<BacdiveSpeciesJson>, Box<dyn Error>> {
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

    let mut bacid: Vec<BacdiveSpeciesJson> = Vec::new();
    for i in bacstring.iter() {
        if i.trim()
            .split(",")
            .map(|x| x.replace("\"", ""))
            .map(|x| x.replace("#", ""))
            .collect::<Vec<_>>()[0]
            == id
            && !i
                .trim()
                .split(",")
                .map(|x| x.replace("\"", ""))
                .map(|x| x.replace("#", ""))
                .collect::<Vec<_>>()[1]
                .is_empty()
        {
            bacid.push(BacdiveSpeciesJson {
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
            .collect::<Vec<_>>()[0]
            == id
            && i.trim()
                .split(",")
                .map(|x| x.replace("\"", ""))
                .map(|x| x.replace("#", ""))
                .collect::<Vec<_>>()[1]
                .is_empty()
        {
            bacid.push(BacdiveSpeciesJson {
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
    Ok(bacid)
}

fn species_write(path: &str, species: &str) -> Result<Vec<BacdiveSpeciesJson>, Box<dyn Error>> {
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

    let mut bacspecies: Vec<BacdiveSpeciesJson> = Vec::new();
    for i in bacstring.iter() {
        if i.trim()
            .split(",")
            .map(|x| x.replace("\"", ""))
            .map(|x| x.replace("#", ""))
            .collect::<Vec<_>>()[1]
            .split(" ")
            .collect::<Vec<_>>()
            .join("-")
            .to_string()
            == species
            && !i
                .trim()
                .split(",")
                .map(|x| x.replace("\"", ""))
                .map(|x| x.replace("#", ""))
                .collect::<Vec<_>>()[1]
                .is_empty()
        {
            bacspecies.push(BacdiveSpeciesJson {
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
            .collect::<Vec<_>>()[1]
            .split(" ")
            .collect::<Vec<_>>()
            .join("-")
            .to_string()
            == species
            && i.trim()
                .split(",")
                .map(|x| x.replace("\"", ""))
                .map(|x| x.replace("#", ""))
                .collect::<Vec<_>>()[1]
                .is_empty()
        {
            bacspecies.push(BacdiveSpeciesJson {
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
    Ok(bacspecies)
}

fn strain_write(path: &str, strain: &str) -> Result<Vec<BacdiveSpeciesJson>, Box<dyn Error>> {
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
